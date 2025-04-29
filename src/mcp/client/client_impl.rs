use crate::RpcId;
use crate::mcp::InitializeParams;
use crate::mcp::IntoMcpRequest;
use crate::mcp::McpMessage;
use crate::mcp::McpRequest;
use crate::mcp::McpResponse;
use crate::mcp::PingParams;
use crate::mcp::client::transport::new_trx_pair;
use crate::mcp::client::transport::{ClientTransport, ClientTrx, CommRx, CommTx};
use crate::mcp::{Error, Result};
use dashmap::DashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::oneshot;

type OneShotRes = oneshot::Sender<McpMessage>;

#[derive(Clone)]
pub struct Client {
	inner: Arc<ClientInner>,
	comm_inner: Option<Arc<CommInner>>,
}

struct ClientInner {
	name: String,
	version: String,
	res_queue: Arc<DashMap<RpcId, OneShotRes>>,
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
			res_queue: Arc::new(DashMap::new()),
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

		self.run_out_rx(out_rx)?;
		self.run_err_rx(err_rx)?;

		// send the initialize
		let init_params = InitializeParams::from_client_info(self.name(), self.version());
		let init_req = init_params.into_mcp_request();
		self.send_request_raw(init_req).await?;

		Ok(())
	}
}

/// Communications
impl Client {
	pub async fn send_request_raw<P>(&self, req: impl Into<McpRequest<P>>) -> Result<McpMessage>
	where
		P: Serialize,
	{
		let req = req.into();

		// -- Build and bind the one shot for the response
		let (tx, rx) = oneshot::channel::<McpMessage>();
		let rpc_id = req.id.clone();
		self.inner.res_queue.insert(rpc_id, tx);

		// -- Send the message
		let msg = serde_json::to_string(&req).map_err(Error::custom_from_err)?;
		self.try_in_tx()?.send(msg).await?;

		// -- Wait for response
		match rx.await {
			Ok(res) => Ok(res),
			Err(err) => Err(Error::custom_from_err(err)),
		}
	}

	pub async fn send_request<REQ, P>(&self, req: REQ) -> crate::mcp::Result<McpResponse<REQ::McpResult>>
	where
		REQ: Into<McpRequest<P>>,
		REQ: IntoMcpRequest<P>,
		P: Serialize,
	{
		let req = req.into();
		// Get the generic/raw McpMessage
		let response = self.send_request_raw(req).await?;
		// Get McpResponse
		let response = response.try_into_response()?;

		let id = response.id;
		let result = response.result;

		let result = serde_json::from_value::<REQ::McpResult>(result).map_err(Error::custom_from_err)?;

		Ok(McpResponse { id, result })
	}
}

/// Public Accessors
impl Client {
	pub fn name(&self) -> &str {
		&self.inner.name
	}

	pub fn version(&self) -> &str {
		&self.inner.version
	}
}

/// Private Accessors
impl Client {
	fn transport(&self) -> Option<&ClientTransport> {
		self.comm_inner.as_ref().map(|t| &t.transport)
	}

	fn try_transport(&self) -> Result<&ClientTransport> {
		let transport = self.transport().ok_or("Client not connected (no transport)")?;
		Ok(transport)
	}

	fn try_in_tx(&self) -> Result<&CommTx> {
		let trans_inner = self.comm_inner.as_ref().ok_or("Client not connected (no transport inner")?;
		let in_tx = &trans_inner.in_tx;
		Ok(in_tx)
	}
}

// region:    --- Runners

impl Client {
	fn run_out_rx(&self, out_rx: CommRx) -> Result<()> {
		let res_queue = self.inner.res_queue.clone();
		tokio::spawn(async move {
			loop {
				match out_rx.recv().await {
					Ok(msg) => {
						let Ok(mcp_message) = serde_json::from_str::<McpMessage>(&msg) else {
							println!("ERROR Parsing received McpMessage {msg}.");
							continue;
						};
						match mcp_message.rpc_id() {
							Some(rpc_id) => match res_queue.remove(rpc_id) {
								Some((_, one_shot)) => {
									one_shot.send(mcp_message);
								}
								None => {
									let val = always_to_string(&mcp_message);
									println!("No matching request for id: {rpc_id} - {val}")
								}
							},
							None => {
								// no id, just print for now
								let val = always_to_string(&mcp_message);
								println!("<<- not a ided message: {val}")
							}
						}
					}
					Err(e) => {
						println!("Error receiving out_rx message: {:?}", e);
						break;
					}
				}
			}
		});

		Ok(())
	}

	fn run_err_rx(&self, err_rx: CommRx) -> Result<()> {
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
}

// endregion: --- Runners

// region:    --- Support

fn always_to_string<T: Serialize + std::fmt::Debug>(val: &T) -> String {
	// Try to serialize using a reference to val, so we don't move it
	match serde_json::to_string(val) {
		Ok(string) => string,
		Err(err) => format!("Error while stringify received message: {:?}. Cause: {}", val, err),
	}
}

// endregion: --- Support
