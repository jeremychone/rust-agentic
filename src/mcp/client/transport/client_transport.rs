use crate::mcp::client::ClientStdioTransportConfig;
use crate::mcp::client::Result;
use crate::mcp::client::server_io_trx::ServerIoTrx;
use crate::mcp::client::transport::ClientStdioTransport;
use derive_more::From;

#[derive(From)]
pub enum ClientTransport {
	StdioTransport(ClientStdioTransport),
}

impl ClientTransport {
	pub(crate) async fn start(&mut self, io_trx: ServerIoTrx) -> Result<()> {
		match self {
			ClientTransport::StdioTransport(transport) => transport.start(io_trx).await?,
		};
		Ok(())
	}

	pub async fn send_to_server(&self, message: impl Into<String>) -> Result<()> {
		match self {
			ClientTransport::StdioTransport(transport) => transport.send_to_server(message).await,
		}
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
