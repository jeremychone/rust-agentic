/// NOTE: HttpTransport is not fully implemented yet.
///       Not to be used (use StdioTransport for now)
pub struct ClientHttpTransportConfig {
	pub url: String,
}

impl ClientHttpTransportConfig {
	pub fn new(url: impl Into<String>) -> Self {
		Self { url: url.into() }
	}
}
