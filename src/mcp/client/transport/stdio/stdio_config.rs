pub struct ClientStdioTransportConfig {
	pub cmd: String,
	pub args: Vec<String>,
	pub current_dir: Option<String>,
}

impl ClientStdioTransportConfig {
	/// Ergonomic new function
	pub fn new<I, S>(cmd: S, args: I, current_dir: Option<String>) -> Self
	where
		S: Into<String>,
		I: IntoIterator,
		I::Item: Into<String>,
	{
		let cmd = cmd.into();
		let args = args.into_iter().map(Into::into).collect();
		Self { cmd, args, current_dir }
	}
}
