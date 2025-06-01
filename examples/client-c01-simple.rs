//! client-c01-simple - Basic MCP Client example
//!
//! With demo MCP Server: https://github.com/modelcontextprotocol/servers/tree/main/src/everything

use agentic::mcp::ListToolsParams;
use agentic::mcp::client::{Client, ClientStdioTransportConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// tracing_subscriber::fmt()
	// 	.with_max_level(tracing::Level::DEBUG)
	// 	.without_time()
	// 	.init();

	// -- Create MCP Client
	let mut client = Client::new("Demo Client", "0.1.0");
	let transport = ClientStdioTransportConfig::new(
		// cmd and args (this MCP Server requires nodejs to be installed)
		"npx",
		["-y", "@modelcontextprotocol/server-everything"],
		None,
	);

	// -- Connect
	client.connect(transport).await?;

	// -- List tools
	let res = client.send_request(ListToolsParams::default()).await?;

	let list_tools = res.result;

	// -- Print tool names
	for tool in list_tools.tools.iter() {
		println!("{}", tool.name);
	}

	Ok(())
}
