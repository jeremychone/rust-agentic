use crate::mcp::client::transport::{ClientHttpTransportConfig, TransportTrx};
use crate::mcp::client::transport::{CommTx, Result};
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use reqwest::header::HeaderValue;
use reqwest::{Response, ResponseBuilderExt};
use std::ops::Index;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error};

pub struct ClientHttpTransport {
	config: Arc<ClientHttpTransportConfig>,
}

/// Lifecyle - start
impl ClientHttpTransport {
	pub(crate) async fn start(&mut self, transport_trx: TransportTrx) -> Result<()> {
		let TransportTrx { in_rx, out_tx, err_tx } = transport_trx;

		// TODO: probably need add cookies support
		let req_client = reqwest::ClientBuilder::new().build()?;

		// -- Sending Request to Server (equivalent of std_in/stdout_out)
		let config = self.config.clone();

		let session_id_holder: Arc<Mutex<Option<String>>> = Arc::default();

		tokio::spawn(async move {
			while let Ok(txt) = in_rx.recv().await {
				// TODO: remove the txt.clone
				let req = req_client
					.post(&config.url)
					.header(reqwest::header::CONTENT_TYPE, "application/json")
					.header(reqwest::header::ACCEPT, "text/event-stream, application/json")
					.body(txt);

				let mut session_holder_guard = session_id_holder.lock().await;
				let (req, holder_sid) = if let Some(session_id) = session_holder_guard.as_ref() {
					let req = req.header("mcp-session-id", session_id);
					(req, Some(session_id))
				} else {
					// TODO: Make sure to do a warn if not initialize event (string loose match)
					(req, None)
				};
				// drop(session_holder_guard);

				// -- Send Request
				let Ok(mut res) = req.send().await else {
					error!("Cannot build MCP SEND REQUEST");
					continue;
				};

				// -- Set and Check mcp-session-id
				let res_session_id = res.headers().get("mcp-session-id").and_then(|v| v.to_str().ok());
				match (holder_sid, res_session_id) {
					(None, Some(session_id)) => {
						*session_holder_guard = Some(session_id.to_string());
					}
					(Some(holder_sid), Some(session_id)) => {
						if holder_sid != session_id {
							error!("MCP Server did not send matching session id. Abort");
							continue;
						}
					}
					_ => (),
				}

				// FIXME: Need handle cases when no content-type or content-type is application/json
				let res_content_type = res.headers().get("content-type").and_then(|v| v.to_str().ok());
				match res_content_type {
					// -- When sse, we treat it as such
					Some("text/event-stream") => {
						process_sse_event(res, &out_tx).await;
					}
					// -- When application/json or no contentype treat it as the json-rpc response
					// NOTE: None because sometime servers (like the everything) seems
					//       to be sending json-rpc error with not content type (but json)
					Some("application/json") | None => {
						let txt = match res.text().await {
							Ok(txt) => txt,
							Err(err) => {
								error!("MCP Response fail to read body - {err}");
								continue;
							}
						};
						if let Err(err) = out_tx.send(txt).await {
							error!(%err, "while sending txt to out_txt.");
							continue;
						}
					}
					Some(other) => {
						error!("MCP Server responded with non supporter content type {other}.");
					}
				}
			}
		});

		Ok(())
	}
}

// region:    --- Support

async fn process_sse_event(res: Response, out_tx: &CommTx) -> Result<()> {
	let mut stream = res.bytes_stream().eventsource();

	while let Some(event) = stream.next().await {
		match event {
			Ok(event) => {
				//
				debug!(
					"mcp sse event received: id={},type={},data_len={}",
					event.id,
					event.event,
					event.data.len()
				);
				if let Err(err) = out_tx.send(event.data).await {
					error!(%err, "while sending txt to out_txt.");
				}
			}
			Err(e) => error!("stream event error occured: {}", e),
		}
	}

	Ok(())
}
// endregion: --- Support

// region:    --- Froms

impl From<ClientHttpTransportConfig> for ClientHttpTransport {
	fn from(config: ClientHttpTransportConfig) -> Self {
		let config = Arc::new(config);
		Self { config }
	}
}

// endregion: --- Froms
