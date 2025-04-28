use crate::mcp::client::ServerIoTrx;
use crate::mcp::client::transport::support::StdioHandles;
use crate::mcp::client::{ClientStdioTransportConfig, Error, Result};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader};
use tokio::process::{ChildStdin, Command};

pub struct ClientStdioTransport {
	config: Arc<ClientStdioTransportConfig>,
	inner: Option<Arc<ClientStdioTransportInner>>,
}

pub struct ClientStdioTransportInner {
	stdio_trx: ServerIoTrx,
	stdio_handles: StdioHandles,
}

/// Lifecycle - start
impl ClientStdioTransport {
	pub(super) async fn start(&mut self, stdio_trx: ServerIoTrx) -> Result<()> {
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
		let stderr_tx = stdio_trx.stderr_tx();
		let stderr_handle = tokio::spawn(async move {
			let reader = BufReader::new(child_stderr);
			let mut lines = reader.lines();

			loop {
				match lines.next_line().await {
					Ok(Some(line)) => {
						println!("->> DEBUG STDERR: {line}");
						if let Err(err) = stderr_tx.send(line).await {
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
			println!("\n->> STDERR Task Ended\n");
		});

		// -- STDOUT (line by line)
		// Read the  child_stdout and send them via stdout_tx to stdout_rx
		let stdout_tx = stdio_trx.out_tx();
		let stdout_handle = tokio::spawn(async move {
			let reader = BufReader::new(child_stdout);
			let mut lines = reader.lines();

			loop {
				match lines.next_line().await {
					Ok(Some(line)) => {
						println!("->> RECEIVED: {line}");
						if let Err(err) = stdout_tx.send(line).await {
							eprintln!("ERROR while sending stdout line. Cause: {err}");
						}
					}
					Ok(None) => {
						eprintln!("readline nothing");
						// End of stream
						break;
					}
					Err(e) => {
						eprintln!("ERROR reading stdout line: {}", e);
						break;
					}
				}
			}
			println!("\n->> STDOUT Task Ended\n");
		});

		// -- STDIN
		// listen the stdin_rx and forward them to child_stdin
		let stdin_rx = stdio_trx.stdin_rx();
		let stdin_handle = tokio::spawn(async move {
			while let Ok(txt) = stdin_rx.recv().await {
				if let Err(err) = send_to_stdin(&mut child_stdin, &txt).await {
					eprintln!("ERROR sending to stdin. Cause: {err}");
					// Decide if the task should terminate on send error
					break;
				}
			}
			println!("\n->> STDIN Task Ended\n");
		});

		// -- Build the ClientTransportController
		let stdio_handles = StdioHandles::new(child, stdin_handle, stdout_handle, stderr_handle);
		self.inner = Some(Arc::new(ClientStdioTransportInner {
			stdio_trx,
			stdio_handles,
		}));

		Ok(())
	}
}

/// Send Event
impl ClientStdioTransport {
	/// Low level send
	pub async fn send_to_server(&self, message: impl Into<String>) -> Result<()> {
		let in_tx = self.try_stdio_trx()?.in_tx();

		in_tx.send(message.into()).await?;

		Ok(())
	}
}

/// Assessors
impl ClientStdioTransport {
	fn try_inner(&self) -> Result<&ClientStdioTransportInner> {
		let inner = self
			.inner
			.as_ref()
			.ok_or_else(|| Error::custom("ClientStdioTransport not started"))?;
		Ok(inner.as_ref())
	}

	fn try_stdio_trx(&self) -> Result<&ServerIoTrx> {
		let inner = self.try_inner()?;
		Ok(&inner.stdio_trx)
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
	// 1. Write payload asynchronously (DO NOT re-serialize)
	println!("->> SENT TO STDIN: -{payload}-");
	if let Err(e) = child_stdin.write_all(payload.as_bytes()).await {
		eprintln!("Error writing payload to stdin: {}", e);
		// Return an error to potentially stop the STDIN loop
		return Err(Error::custom(format!("Error writing payload to stdin: {}", e)));
	}

	// 2. Add a newline asynchronously
	if let Err(e) = child_stdin.write_all(b"\n").await {
		eprintln!("Error writing newline to stdin: {}", e);
		return Err(Error::custom(format!("Error writing newline to stdin: {}", e)));
	}
	// 3. Flush the stdin buffer async
	if let Err(e) = child_stdin.flush().await {
		eprintln!("Error flushing stdin: {}", e);
		return Err(Error::custom(format!("Error flushing stdin: {}", e)));
	}
	Ok(())
}

// endregion: --- Support
