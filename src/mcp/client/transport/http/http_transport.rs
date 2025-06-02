use crate::mcp::client::transport::Result;
use crate::mcp::client::transport::{ClientHttpTransportConfig, TransportTrx};
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use reqwest::ResponseBuilderExt;
use reqwest::header::HeaderValue;
use std::ops::Index;
use std::sync::Arc;
use tracing::{debug, error};

pub struct ClientHttpTransport {
	config: Arc<ClientHttpTransportConfig>,
	inner: Option<Arc<ClientHttpTransportInner>>,
}

pub struct ClientHttpTransportInner {
	// NOTE: Not sure how to capture it for now.
	mcp_session_id: Option<String>,
}

/// Lifecyle - start
impl ClientHttpTransport {
	pub(crate) async fn start(&mut self, transport_trx: TransportTrx) -> Result<()> {
		let TransportTrx { in_rx, out_tx, err_tx } = transport_trx;

		// TODO: probably need add cookies support
		let req_client = reqwest::ClientBuilder::new().build()?;

		// -- Sending Request to Server
		let config = self.config.clone();
		tokio::spawn(async move {
			while let Ok(txt) = in_rx.recv().await {
				let header = HeaderValue::from_static("application/json");
				// TODO: remove the txt.clone
				let req = req_client
					.post(&config.url)
					.header(reqwest::header::CONTENT_TYPE, header)
					.header(reqwest::header::ACCEPT, "text/event-stream, application/json")
					.body(txt.clone());

				// --
				let Ok(mut res) = req.send().await else {
					error!("Cannot build MCP SEND REQUEST");
					continue;
				};

				debug!("MCP Response headers: {:?}", res.headers());

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
		Self { config, inner: None }
	}
}

// endregion: --- Froms
