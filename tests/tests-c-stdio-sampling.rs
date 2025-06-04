mod support;

use agentic::mcp::{CallToolParams, ListToolsParams};
use support::Result;

#[tokio::test]
async fn test_c_stdio_sampling_simple() -> Result<()> {
	// -- Fixtures & Setup
	let client = support::mock_new_stdio_client_and_connect().await?;

	// -- Trigger the sampling
	let params = CallToolParams::new("sampleLLM").append_argument("prompt", "Why is the sky red?");

	println!("->> BEFORE\n\n");
	let res: agentic::mcp::CallToolResult = client.send_request(params).await?.result;
	println!("\n\n->> AFTER");
	let pretty = serde_json::to_string_pretty(&res)?;

	// -- Exec

	// -- Check

	// -- XP

	Ok(())
}
