mod support;

use support::Result;

#[tokio::test]
async fn test_c_stdio_livecycle_connect() -> Result<()> {
	// -- Fixtures & Setup
	let (mut client, transport) = support::mock_new_stdio_client()?;

	// -- Exec
	let init_res = client.connect(transport).await?.result;

	// -- Check
	let server_info_name = &init_res.server_info.name;
	assert_eq!(server_info_name, "example-servers/everything");

	Ok(())
}
