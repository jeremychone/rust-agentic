use crate::RpcId;
use crate::mcp::CreateMessageParams;
use crate::mcp::InitializeParams;
use crate::mcp::InitializeResult;
use crate::mcp::IntoMcpRequest;
use crate::mcp::McpMessage;
use crate::mcp::McpRequest;
use crate::mcp::McpResponse;
use crate::mcp::SamplingMessage;
use crate::mcp::client::IntoClientTransport;
use crate::mcp::client::SamplingHandlerAsyncFn;
use crate::mcp::client::sampling_handler::IntoSamplingHandlerAsyncFn;
use crate::mcp::client::transport::new_trx_pair;
use crate::mcp::client::transport::{ClientTransport, ClientTrx, CommRx, CommTx};
use crate::mcp::support::truncate;
use crate::mcp::{Error, Result};
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

type OneShotRes = oneshot::Sender<McpMessage>;
type ResQueue = Arc<DashMap<RpcId, OneShotRes>>;

#[derive(Clone)]
pub struct Client {
	inner: Arc<ClientInner>,
	comm_inner: Option<Arc<CommInner>>,
	sampling_handler: Option<Arc<Box<dyn SamplingHandlerAsyncFn + 'static>>>,
	s2c_mcp_requests_tx: Option<flume::Sender<McpRequest>>,
}

struct ClientInner {
	name: String,
	version: String,
	// This is the DashMap of the rpc_id: OneShot<res>
	res_queue: ResQueue,
}

struct CommInner {
	#[allow(unused)]
	transport: ClientTransport,
	c2s_tx: CommTx,
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
			sampling_handler: None,
			s2c_mcp_requests_tx: None,
		}
	}

	/// Connects the client using a transport configuration.
	///
	/// Accepts any type that implements `IntoClientTransport`, such as `ClientStdioTransportConfig`.
	pub async fn connect(
		&mut self,
		transport_source: impl IntoClientTransport,
	) -> Result<McpResponse<InitializeResult>> {
		// Check not already connected
		if self.comm_inner.is_some() {
			return Err(
				"Client already connected. Reconnect not supported for now.\nRecommendation: Start a new client".into(),
			);
		}

		// -- Create the Trx Pair
		let (client_trx, transport_trx) = new_trx_pair();

		// Convert the input into the internal ClientTransport using the trait method
		let mut transport: ClientTransport = transport_source.into_client_transport();

		// Start the transport
		transport.start(transport_trx).await?;
		let transport = transport; // no need to mut anymore

		let ClientTrx {
			c2s_tx,
			s2c_rx,
			s2c_aux_rx,
		} = client_trx;
		self.comm_inner = Some(CommInner { transport, c2s_tx }.into());

		// -- Run Typed MCP Requests
		let (s2c_mcp_requests_tx, s2c_mcp_requests_rx) = flume::unbounded::<McpRequest>();
		self.s2c_mcp_requests_tx = Some(s2c_mcp_requests_tx);
		self.run_server_requests(s2c_mcp_requests_rx)?;

		// -- Run Transport messages
		self.run_s2c_rx(s2c_rx)?;
		self.run_s2c_aux_rx(s2c_aux_rx)?;

		// send the initialize
		let init_params = InitializeParams::from_client_info(self.name(), self.version());
		let res = self.send_request(init_params).await?;

		Ok(res)
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
		let rpc_id = &req.id;
		let method = &req.method;
		debug!(rpc_id = %rpc_id, method = %method, "Sending RPC Request");
		let msg = serde_json::to_string(&req).map_err(Error::custom_from_err)?;
		self.try_c2s_tx()?.send(msg).await?;

		// -- Wait for response
		match rx.await {
			Ok(res) => Ok(res),
			Err(err) => Err(Error::custom_from_err(err)),
		}
	}

	pub async fn send_request<REQ, P>(&self, req: REQ) -> Result<McpResponse<REQ::McpResult>>
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

	pub async fn send_response<R>(&self, mcp_response: McpResponse<R>) -> Result<()>
	where
		R: Serialize,
	{
		let in_tx = self.try_c2s_tx()?;
		let payload = serde_json::to_string(&mcp_response).map_err(Error::custom_from_err)?;
		if let Err(err) = in_tx.send(payload).await {
			error!("Fail to send in_tx send_response_raw. Cause {err}");
			return Err(err.into());
		};

		Ok(())
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

/// Handlers
impl Client {
	pub fn register_sampling_handler(&mut self, sampling_handler: impl IntoSamplingHandlerAsyncFn) {
		let sampling_handler = sampling_handler.into_sampling_handler();
		self.sampling_handler = Some(sampling_handler);
	}

	pub async fn exec_sampling_handler(
		&self,
		create_message_req: McpRequest<CreateMessageParams>,
	) -> Result<McpResponse<SamplingMessage>> {
		// FIXME: do the work
		Err("client::exec_sampling_handler not implemented".into())
	}
}

/// Private Accessors
impl Client {
	fn try_c2s_tx(&self) -> Result<&CommTx> {
		let trans_inner = self.comm_inner.as_ref().ok_or("Client not connected (no transport inner")?;
		let in_tx = &trans_inner.c2s_tx;
		Ok(in_tx)
	}

	fn try_s2c_mcp_requests_tx(&self) -> Result<&flume::Sender<McpRequest>> {
		let tx = self
			.s2c_mcp_requests_tx
			.as_ref()
			.ok_or("Client not connected (no s2c_mcp_requests_tx)")?;
		Ok(tx)
	}
}

/// Runners
impl Client {
	fn run_s2c_rx(&self, s2c_rx: CommRx) -> Result<()> {
		let res_queue = self.inner.res_queue.clone();
		let try_s2c_mcp_requests_tx = self.try_s2c_mcp_requests_tx()?.clone();
		tokio::spawn(async move {
			loop {
				match s2c_rx.recv().await {
					Ok(msg) => {
						let Ok(mcp_message) = serde_json::from_str::<McpMessage>(&msg) else {
							error!(message = %msg, "Parsing received McpMessage");
							continue;
						};
						match mcp_message {
							McpMessage::Response(mcp_response) => process_mcp_response(mcp_response, &res_queue),
							McpMessage::Request(mcp_request) => {
								match try_s2c_mcp_requests_tx.send_async(mcp_request).await {
									Ok(_) => (),
									Err(err) => {
										error!("error sending to s2c_mcp_requests_tx. Cause: {err} ")
									}
								}
							}
							McpMessage::Notification(mcp_notification) => {
								warn!("MCP Notification in out_rx not supported yet")
							}

							McpMessage::Error(mcp_error) => warn!("MCP Error in out_rx not supported yet"),
						}
					}
					Err(e) => {
						error!(%e, "Receiving out_rx message");
						break;
					}
				}
			}
		});

		Ok(())
	}

	/// Handle the server requests
	/// TODO: For now, only support the sampling_handler (no routing)
	fn run_server_requests(&self, s2c_mcp_request_rx: flume::Receiver<McpRequest>) -> Result<()> {
		let sampling_handler = self.sampling_handler.clone();
		let c2s_tx = self.try_c2s_tx()?.clone();

		tokio::spawn(async move {
			loop {
				match s2c_mcp_request_rx.recv_async().await {
					Ok(mcp_request) => {
						// TODO: Today assuming register sampling handler before connect.
						//       Otherwise, need to use mutex to get the latest sampling_handler
						let Some(sampling_handler) = sampling_handler.as_ref() else {
							error!("This client does not have any sampling. Cannot process event");
							continue;
						};

						// NOTE - Today no routing, assuming server request can only be Sampling Request

						let Some(sampling_request_params) = mcp_request.params else {
							error!("McpRequest is not Sampling request. No params");
							continue;
						};

						let Ok(sampling_request_params) =
							serde_json::from_value::<CreateMessageParams>(sampling_request_params)
						else {
							error!("McpRequest is not a Sampling request. Params fail parsing as CreateMessageParams");
							continue;
						};

						match sampling_handler.exec_fn(sampling_request_params).await {
							Ok(res) => {
								let res = McpResponse {
									id: mcp_request.id,
									result: res,
								};
								let payload = match serde_json::to_string(&res) {
									Ok(res) => res,
									Err(err) => {
										error!("While serializing McpResponse for c2s. {res:?}");
										continue;
									}
								};
								c2s_tx.send(payload).await;
							}
							Err(err) => {
								//
								error!("Error processing sampling. Cause: {err}")
							}
						};
					}
					Err(err) => error!("Cannot rx from s2c_mcp_request_rx. Cause: {err}"),
				}
			}
		});

		Ok(())
	}

	fn run_s2c_aux_rx(&self, err_rx: CommRx) -> Result<()> {
		tokio::spawn(async move {
			loop {
				match err_rx.recv().await {
					Ok(msg) => warn!(io_err = %msg,"io_err"),
					Err(e) => {
						info!(%e, "aux_rx dropped not needed");
						break;
					}
				}
			}
		});

		Ok(())
	}
}

// region:    --- Support

fn process_mcp_response(mcp_res: McpResponse, res_queue: &ResQueue) {
	let rpc_id = mcp_res.id.clone();

	// FIXME: Need to fix when it's a McpRequest from mcp server (e.g., sampling)
	debug!(rpc_id = %rpc_id, "Received RPC Response");

	match res_queue.remove(&rpc_id) {
		Some((rpc_id, one_shot)) => match one_shot.send(mcp_res.into()) {
			Ok(_) => (),
			Err(_) => error!(rpc_id = %rpc_id, "Cannot send one_shot"),
		},
		None => {
			let payload = always_to_string(&mcp_res);
			error!(rpc_id = %rpc_id, payload_excerpt = %truncate(&payload, 256), "No matching request that id")
		}
	}
}

fn always_to_string<T: Serialize + std::fmt::Debug>(val: &T) -> String {
	// Try to serialize using a reference to val, so we don't move it
	match serde_json::to_string(val) {
		Ok(string) => string,
		Err(err) => format!("Error while stringify received message: {:?}. Cause: {}", val, err),
	}
}

// endregion: --- Support
