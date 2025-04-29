use crate::mcp::InitializeParams;
use crate::mcp::IntoMcpRequest;
use crate::mcp::McpRequest;
use crate::mcp::PingParams;
use crate::mcp::client::coms_trx::ClientTrx;
use crate::mcp::client::coms_trx::CommRx;
use crate::mcp::client::coms_trx::CommTx;
use crate::mcp::client::coms_trx::new_trx_pair;
use crate::mcp::client::transport::ClientTransport;
use crate::mcp::client::{Error, Result};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
	inner: Arc<ClientInner>,
	comm_inner: Option<Arc<CommInner>>,
}

struct ClientInner {
	name: String,
	version: String,
}

struct CommInner {
	transport: ClientTransport,
	in_tx: CommTx,
}

/// Constructors & Connect
impl Client {
	pub fn new(client_name: impl Into<String>, client_version: impl Into<String>) -> Client {
		// -- Create the ClientTransportController
		let info_inner = ClientInner {
			name: client_name.into(),
			version: client_version.into(),
		};
		Self {
			inner: info_inner.into(),
			comm_inner: None,
		}
	}

	pub async fn connect(&mut self, transport: impl Into<ClientTransport>) -> Result<()> {
		// Check not already connected
		if self.comm_inner.is_some() {
			return Err(
				"Client already connected. Reconnect not supported for now.\nRecommendation: Start a new client".into(),
			);
		}

		// -- Create the Trx Pair
		let (client_trx, transport_trx) = new_trx_pair();

		// Start the transport
		let mut transport: ClientTransport = transport.into();
		transport.start(transport_trx).await?;
		let transport = transport; // no need to mut anymore

		let ClientTrx { in_tx, out_rx, err_rx } = client_trx;
		self.comm_inner = Some(CommInner { transport, in_tx }.into());

		run_out_rx(out_rx)?;
		run_err_rx(err_rx)?;

		// send the initialize
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
		self.try_in_tx()?.send(msg).await?;

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
		self.comm_inner.as_ref().map(|t| &t.transport)
	}

	pub fn try_transport(&self) -> Result<&ClientTransport> {
		let transport = self.transport().ok_or("Client not connected (no transport)")?;
		Ok(transport)
	}

	pub fn try_in_tx(&self) -> Result<&CommTx> {
		let trans_inner = self.comm_inner.as_ref().ok_or("Client not connected (no transport inner")?;
		let in_tx = &trans_inner.in_tx;
		Ok(in_tx)
	}
}

// region:    --- Runners

fn run_out_rx(out_rx: CommRx) -> Result<()> {
	tokio::spawn(async move {
		loop {
			match out_rx.recv().await {
				Ok(msg) => println!("<<- {}", msg),
				Err(e) => {
					println!("Error receiving out_rx message: {:?}", e);
					break;
				}
			}
		}
	});

	Ok(())
}

fn run_err_rx(err_rx: CommRx) -> Result<()> {
	tokio::spawn(async move {
		loop {
			match err_rx.recv().await {
				Ok(msg) => println!("ERR: {}", msg),
				Err(e) => {
					println!("Error receiving err_rx message: {:?}", e);
					break;
				}
			}
		}
	});

	Ok(())
}

// endregion: --- Runners
