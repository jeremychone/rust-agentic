use crate::mcp::client::transport::Result;
use crate::mcp::client::transport::{ClientHttpTransportConfig, TransportTrx};
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use reqwest::ResponseBuilderExt;
use reqwest::header::HeaderValue;
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
				if !res.headers().contains_key("content-type") {
					let headers = res.headers().clone();
					let txt = res.text().await.unwrap_or_else(|_| "NO CONTENT".to_string());
					error!("NO CONTENT TYPE.\nHeaders: {:?}\nBody:\n{}", headers, txt);
					continue;
				}

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
			}
		});

		Ok(())
	}
}

// region:    --- Froms

impl From<ClientHttpTransportConfig> for ClientHttpTransport {
	fn from(config: ClientHttpTransportConfig) -> Self {
		let config = Arc::new(config);
		Self { config }
	}
}

// endregion: --- Froms
