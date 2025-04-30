use tokio::process::Child;
use tokio::task::JoinHandle;

// region:    --- StdioHandles

#[allow(unused)]
pub(super) struct StdioHandles {
	child: Child,
	stdin: JoinHandle<()>,
	stdout: JoinHandle<()>,
	stderr: JoinHandle<()>,
}

impl StdioHandles {
	pub fn new(child: Child, stdin: JoinHandle<()>, stdout: JoinHandle<()>, stderr: JoinHandle<()>) -> Self {
		Self {
			child,
			stdin,
			stdout,
			stderr,
		}
	}
}
// endregion: --- StdioHandles
