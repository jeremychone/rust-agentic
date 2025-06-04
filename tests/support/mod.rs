// region:    --- Modules

use agentic::mcp::client::{Client, ClientStdioTransportConfig};

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

// endregion: --- Modules

pub async fn mock_new_stdio_client_and_connect() -> Result<Client> {
	let (mut client, transport_config) = mock_new_stdio_client()?;

	// -- Connect
	client.connect(transport_config).await?;

	Ok(client)
}

pub fn mock_new_stdio_client() -> Result<(Client, ClientStdioTransportConfig)> {
	let mut client = Client::new("Demo Client", "0.1.0");
	let transport_config = ClientStdioTransportConfig::new(
		// cmd and args (this MCP Server requires nodejs to be installed)
		"npx",
		["-y", "@modelcontextprotocol/server-everything"],
		None,
	);

	Ok((client, transport_config))
}
