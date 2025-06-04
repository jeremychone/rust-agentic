mod support;

use agentic::mcp::{CallToolParams, ListToolsParams};
use support::Result;

// TODO: To implement
// #[tokio::test]
async fn test_c_stdio_sampling_simple() -> Result<()> {
	// -- Fixtures & Setup
	let client = support::mock_new_stdio_client_and_connect().await?;

	// -- Trigger the sampling
	let params = CallToolParams::new("sampleLLM").append_argument("prompt", "Why is the sky red?");

	let res: agentic::mcp::CallToolResult = client.send_request(params).await?.result;

	let pretty = serde_json::to_string_pretty(&res)?;

	Ok(())
}
