use crate::mcp::InitializeParams;
use crate::mcp::IntoMcpRequest;
use crate::mcp::McpRequest;
use crate::mcp::PingParams;
use crate::mcp::client::ServerIoTrx;
use crate::mcp::client::transport::ClientTransport;
use crate::mcp::client::{Error, Result};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
	inner: Arc<ClientInner>,
	transport_inner: Option<Arc<ClientTransport>>,
}

struct ClientInner {
	name: String,
	version: String,
	stdio_trx: ServerIoTrx,
}

struct TransportInner {
	transport: ClientTransport,
}

/// Constructors & Connect
impl Client {
	pub fn new(client_name: impl Into<String>, client_version: impl Into<String>) -> Client {
		// -- Create the ClientTransportController
		let stdio_trx = ServerIoTrx::default();
		let info_inner = ClientInner {
			name: client_name.into(),
			version: client_version.into(),
			stdio_trx,
		};
		Self {
			inner: info_inner.into(),
			transport_inner: None,
		}
	}

	pub async fn connect(&mut self, transport: impl Into<ClientTransport>) -> Result<()> {
		// Check not already connected
		if self.transport_inner.is_some() {
			return Err(
				"Client already connected. Reconnect not supported for now.\nRecommendation: Start a new client".into(),
			);
		}

		// Start the transport
		let mut transport: ClientTransport = transport.into();
		transport.start(self.inner.stdio_trx.clone()).await?;
		let transport = transport; // no need to mut anymore

		self.transport_inner = Some(transport.into());

		// send the initiaize
		let init_params = InitializeParams::from_client_info(self.name(), self.version());
		let init_req = init_params.into_mcp_request();
		self.send_request(init_req).await?;
		println!("->> after initialize req");

		// sleep 500 ms
		// tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;

		Ok(())
	}
}

/// Communications
impl Client {
	pub async fn send_request<P>(&self, req: McpRequest<P>) -> Result<()>
	where
		P: Serialize,
	{
		let msg = serde_json::to_string(&req).map_err(Error::custom_from_err)?;
		self.try_transport()?.send_to_server(msg).await?;

		Ok(())
	}
}

/// Accessors
impl Client {
	pub fn name(&self) -> &str {
		&self.inner.name
	}

	pub fn version(&self) -> &str {
		&self.inner.version
	}

	pub fn transport(&self) -> Option<&ClientTransport> {
		self.transport_inner.as_ref().map(|t| t.as_ref())
	}

	pub fn try_transport(&self) -> Result<&ClientTransport> {
		let transport = self.transport().ok_or("Client not connected yet")?;
		Ok(transport)
	}
}
