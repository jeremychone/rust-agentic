use agentic::mcp::client::{Client, ClientStdioTransportConfig};
use agentic::mcp::{CallToolParams, CreateMessageParams, CreateMessageResult, McpResponse, SamplingMessage};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

// doc: https://modelcontextprotocol.io/docs/concepts/sampling

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::INFO)
		.without_time()
		.init();

	// -- Setup
	let mut client = Client::new("Demo Client", "0.1.0");

	// -- Register the sampling
	let some_stuff = 123;
	client.register_sampling_handler(
		async move |_params: CreateMessageParams| -> agentic::mcp::Result<CreateMessageResult> {
			let message_result = CreateMessageResult::new_assistant(
				//
				"The sky appears red, especially at sunrise or sunset, because sunlight passes through more of the Earth's atmosphere, scattering shorter blue wavelengths and allowing longer red wavelengths to dominate.",
				"mock-model-xp",
			);
			Ok(message_result)
		},
	);

	// -- Transport Config
	let transport_config = ClientStdioTransportConfig::new(
		// cmd and args (this MCP Server requires nodejs to be installed)
		"npx",
		["-y", "@modelcontextprotocol/server-everything"],
		None,
	);

	client.connect(transport_config).await?;

	let client = client;

	let client_for_req = client.clone();
	// -- Trigger the sampling
	let params = CallToolParams::new("sampleLLM").append_argument("prompt", "Why is the sky red?");

	let res = client_for_req.send_request(params).await?;

	let res = serde_json::to_string_pretty(&res)?;
	println!("Tool response (after sampling):\n{res}");

	// sleep 1 sec
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;

	Ok(())
}
