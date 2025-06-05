use agentic::mcp::client::{Client, ClientStdioTransportConfig};
use agentic::mcp::{
	CallToolParams, CreateMessageParams, CreateMessageResult, ListToolsParams, McpRequest, McpResponse, SamplingMessage,
};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

// doc: https://modelcontextprotocol.io/docs/concepts/sampling

// // This function is no longer needed as we will use an async closure
// async fn app_sampling_handler(mcp_sample_request: CreateMessageParams) -> agentic::mcp::Result<SamplingMessage> {
// 	// todo!("app_sampling_handler not implemented yet")
// 	Err(agentic::mcp::Error::custom("app_sampling_handler not implemented yet"))
// }

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::WARN)
		.without_time()
		.init();

	// -- Setup
	let mut client = Client::new("Demo Client", "0.1.0");
	let transport_config = ClientStdioTransportConfig::new(
		// cmd and args (this MCP Server requires nodejs to be installed)
		"npx",
		["-y", "@modelcontextprotocol/server-everything"],
		None,
	);
	client.connect(transport_config).await?;

	client.register_sampling_handler(
		async move |_params: CreateMessageParams| -> agentic::mcp::Result<SamplingMessage> {
			println!("Async closure sampling handler called.");
			Err(agentic::mcp::Error::custom(
				"async closure - actual sampling logic not implemented yet",
			))
		},
	);

	let client = client;

	let client_for_req = client.clone();
	tokio::spawn(async move {
		// -- Trigger the sampling
		let params = CallToolParams::new("sampleLLM").append_argument("prompt", "Why is the sky red?");

		println!("\n\nBEFORE\n\n");
		let res = match client_for_req.send_request(params).await {
			Ok(res) => res,
			Err(err) => {
				println!("ERROR when sending sampleLLM - {err}");
				return;
			}
		};
		println!("\n\nAFTER");

		let pretty = serde_json::to_string_pretty(&res.result.content).unwrap_or_else(|_| "FAIL CONTENT".to_string());
		println!("Response:\n{pretty}");
	});

	// sleep 1 sec
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;

	// -- Send mock sample result
	let message_result = CreateMessageResult::new_assistant(
		//
		"The sky appears red, especially at sunrise or sunset, because sunlight passes through more of the Earth's atmosphere, scattering shorter blue wavelengths and allowing longer red wavelengths to dominate.",
		"mock-model-xp",
	);

	let mcp_res = McpResponse {
		id: 0.into(),
		result: message_result,
	};
	client.send_response(mcp_res).await?;

	tokio::time::sleep(std::time::Duration::from_secs(1)).await;

	Ok(())
}
