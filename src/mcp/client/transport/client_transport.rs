use super::Result;
use super::comm_trx::TransportTrx;
use crate::mcp::client::ClientStdioTransportConfig;
use crate::mcp::client::transport::ClientStdioTransport;
use derive_more::From;

#[derive(From)]
pub enum ClientTransport {
	StdioTransport(ClientStdioTransport),
}

impl ClientTransport {
	pub(crate) async fn start(&mut self, transport_trx: TransportTrx) -> Result<()> {
		match self {
			ClientTransport::StdioTransport(transport) => transport.start(transport_trx).await?,
		};
		Ok(())
	}
}

// region:    --- Froms

impl From<ClientStdioTransportConfig> for ClientTransport {
	fn from(config: ClientStdioTransportConfig) -> Self {
		let transport = ClientStdioTransport::from(config);
		transport.into()
	}
}

// endregion: --- Froms
