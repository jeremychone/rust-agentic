//! Simple way to call a MCP Tools
//!
//! With demo MCP Server: https://github.com/modelcontextprotocol/servers/tree/main/src/everything

use agentic::mcp::CallToolParams;
use agentic::mcp::client::{Client, ClientStdioTransportConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::WARN)
		.without_time()
		.init();

	// -- Create MCP Client
	let mut client = Client::new("Demo Client", "0.1.0");
	let transport_config = ClientStdioTransportConfig::new(
		// cmd and args (this MCP Server requires nodejs to be installed)
		"npx",
		["-y", "@modelcontextprotocol/server-everything"],
		None,
	);

	// -- Connect
	client.connect(transport_config).await?;

	// -- Call echo
	// Build the params
	let params = CallToolParams::new("echo").append_argument("message", "Hello Live Coding");
	// Call
	let res = client.send_request(params).await?;
	println!("Calling echo tool:\n{res:#?}");

	// -- Call add
	// Build the params
	let params = CallToolParams::new("add")
		//
		.append_argument("a", 1)
		.append_argument("b", 2.5);
	let res = client.send_request(params).await?;
	println!("Calling add tool:\n{res:#?}");

	Ok(())
}
