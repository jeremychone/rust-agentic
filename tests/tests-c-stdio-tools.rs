mod support;

use agentic::mcp::{CallToolParams, ListToolsParams};
use support::Result;

#[tokio::test]
async fn test_c_stdio_tools_list() -> Result<()> {
	// -- Fixtures & Setup
	let fx_names =
		"echo,add,printEnv,longRunningOperation,sampleLLM,getTinyImage,annotatedMessage,getResourceReference";
	let client = support::mock_new_stdio_client_and_connect().await?;

	// -- Exec
	let res = client.send_request(ListToolsParams::default()).await?;

	// -- Check
	let tools = res.result.tools;
	let tool_names = tools.iter().map(|t| t.name.as_str()).collect::<Vec<&str>>();
	let tool_names = tool_names.join(",");
	assert_eq!(tool_names, fx_names);

	Ok(())
}

#[tokio::test]
async fn test_c_stdio_tools_add() -> Result<()> {
	// -- Fixtures & Setup
	let fx_message = "The sum of 1 and 2.5 is 3.5.";
	let client = support::mock_new_stdio_client_and_connect().await?;

	// -- Exec
	let params = CallToolParams::new("add")
		//
		.append_argument("a", 1)
		.append_argument("b", 2.5);
	let res = client.send_request(params).await?;

	// -- Check
	let msg = res
		.result
		.content
		.first()
		.ok_or("Should have 1 message")?
		.as_text()
		.ok_or("Should have text message")?;
	assert_eq!(msg, fx_message);

	Ok(())
}
