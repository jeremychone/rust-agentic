use crate::mcp::Error;
use crate::mcp::transport::Result;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader}; // Tokio IO traits
use tokio::process::{Child, ChildStdin, Command}; // Tokio Command
use tokio::time::{Duration, sleep}; // Tokio time

/// NOT IMPLEMENTED YET
pub struct StdioTransport {
	cmd: String,
	args: Vec<String>,
	current_dir: Option<String>,
}

impl StdioTransport {
	pub fn new(cmd: String, args: Vec<String>, current_dir: Option<String>) -> Self {
		Self { cmd, args, current_dir }
	}
}

impl StdioTransport {
	pub async fn start(self) -> Result<()> {
		// -- Build the command
		let mut cmd = Command::new(&self.cmd);
		if let Some(ref dir) = self.current_dir {
			cmd.current_dir(dir);
		}
		if !self.args.is_empty() {
			let args = self.args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
			cmd.args(args);
		}
		cmd.stdin(std::process::Stdio::piped()) // Use std pipe for Tokio Command setup
			.stdout(std::process::Stdio::piped())
			.stderr(std::process::Stdio::piped())
			.kill_on_drop(true); // Ensure child is killed if `Child` struct is dropped

		// -- Run the command
		let mut child = cmd.spawn().map_err(Error::custom_from_err);

		Ok(())
	}
}
