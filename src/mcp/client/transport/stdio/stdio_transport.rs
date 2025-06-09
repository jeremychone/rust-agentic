use crate::mcp::client::transport::stdio::stdio_config::ClientStdioTransportConfig;
use crate::mcp::client::transport::support::StdioHandles;
use crate::mcp::client::transport::{Error, Result, TransportTrx};
use crate::mcp::support::truncate;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};
use tokio::process::{ChildStdin, Command};
use tracing::{debug, error, info};

pub struct ClientStdioTransport {
	config: Arc<ClientStdioTransportConfig>,
	inner: Option<Arc<ClientStdioTransportInner>>,
}

pub struct ClientStdioTransportInner {
	#[allow(unused)]
	stdio_handles: StdioHandles,
}

/// Lifecycle - start
impl ClientStdioTransport {
	pub(crate) async fn start(&mut self, transport_trx: TransportTrx) -> Result<()> {
		let TransportTrx {
			c2s_rx,
			s2c_tx,
			s2c_aux_tx,
		} = transport_trx;

		// -- Build the command
		let mut cmd = Command::new(&self.config.cmd);
		if let Some(ref dir) = self.config.current_dir {
			cmd.current_dir(dir);
		}
		if !self.config.args.is_empty() {
			cmd.args(&self.config.args);
		}
		cmd.stdin(std::process::Stdio::piped()) // Use std pipe for Tokio Command setup
			.stdout(std::process::Stdio::piped())
			.stderr(std::process::Stdio::piped())
			.kill_on_drop(true); // Ensure child is killed if `Child` struct is dropped

		// -- Run the command

		let mut child = cmd.spawn().map_err(Error::custom_from_err)?;

		let mut child_stdin = child.stdin.take().expect("Failed to get stdin");
		let child_stdout = child.stdout.take().expect("Failed to get stdout");
		let child_stderr = child.stderr.take().expect("Failed to get stderr");

		// -- STDERR  (line by line)
		// Read the child_stderr and send them via stdout_tx to stdout_rx
		let stderr_handle = tokio::spawn(async move {
			let reader = BufReader::new(child_stderr);
			let mut lines = reader.lines();

			loop {
				match lines.next_line().await {
					Ok(Some(line)) => {
						if let Err(err) = s2c_aux_tx.send(line).await {
							eprintln!("ERROR while sending stderr line. Cause: {err}");
							// Decide if the task should terminate on send error
							break;
						}
					}
					Ok(None) => {
						// End of stream
						break;
					}
					Err(e) => {
						eprintln!("ERROR reading stderr line: {}", e);
						break;
					}
				}
			}
			info!("STDERR Task Ended");
		});

		// -- STDOUT (line by line)
		// Read the  child_stdout and send them via stdout_tx to stdout_rx
		let stdout_handle = tokio::spawn(async move {
			let reader = BufReader::new(child_stdout);
			let mut lines = reader.lines();

			loop {
				match lines.next_line().await {
					Ok(Some(line)) => {
						debug!(payload_excerpt = %truncate(&line, 64), "message received");
						if let Err(err) = s2c_tx.send(line).await {
							error!(%err, "while sending stdout line");
						}
					}
					Ok(None) => {
						info!("stdout readline nothing .. end ");
					}
					Err(e) => {
						error!("Error reading stdout line: {}", e);
						break;
					}
				}
			}
			info!("STDOUT Task Ended");
		});

		// -- STDIN
		// listen the stdin_rx and forward them to child_stdin
		let stdin_handle = tokio::spawn(async move {
			while let Ok(txt) = c2s_rx.recv().await {
				if let Err(err) = send_to_stdin(&mut child_stdin, &txt).await {
					error!("ERROR sending to stdin. Cause: {err}");
					// Decide if the task should terminate on send error
					break;
				}
			}
			info!("STDIN Task Ended");
		});

		// -- Build the ClientTransportController
		let stdio_handles = StdioHandles::new(child, stdin_handle, stdout_handle, stderr_handle);
		self.inner = Some(Arc::new(ClientStdioTransportInner { stdio_handles }));

		Ok(())
	}
}

// region:    --- Froms

impl From<ClientStdioTransportConfig> for ClientStdioTransport {
	fn from(config: ClientStdioTransportConfig) -> Self {
		let config = Arc::new(config);
		Self { config, inner: None }
	}
}

// endregion: --- Froms

// region:    --- Support

async fn send_to_stdin(child_stdin: &mut ChildStdin, payload: &str) -> Result<()> {
	debug!(payload_excerpt = %truncate(payload, 64), "sending message");

	// 1. Write payload asynchronously (DO NOT re-serialize)
	if let Err(e) = child_stdin.write_all(payload.as_bytes()).await {
		error!(%e, payload_excerpt = %truncate(payload, 256), "failed to write to stdin");
		// Return an error to potentially stop the STDIN loop
		return Err(Error::custom(format!("Error writing payload to stdin: {}", e)));
	}

	// 2. Add a newline asynchronously
	if let Err(e) = child_stdin.write_all(b"\n").await {
		error!(%e, payload_excerpt = %truncate(payload, 256), "failed to write new line to stdin");
		return Err(Error::custom(format!("Error writing newline to stdin: {}", e)));
	}
	// 3. Flush the stdin buffer async
	if let Err(e) = child_stdin.flush().await {
		error!(%e, payload_excerpt = %truncate(payload, 256), "error flushing");
		return Err(Error::custom(format!("Error flushing stdin: {}", e)));
	}
	Ok(())
}

// endregion: --- Support
