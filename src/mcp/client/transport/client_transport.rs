use super::Result;
use super::comm_trx::TransportTrx;
use crate::mcp::client::transport::ClientHttpTransport;
use crate::mcp::client::transport::stdio::ClientStdioTransport;
use crate::mcp::client::{self, ClientStdioTransportConfig};
use crate::mcp::client::{ClientHttpTransportConfig, IntoClientTransport};
use derive_more::From;

#[derive(From)]
pub enum ClientTransport {
	StdioTransport(ClientStdioTransport),
	HttpTransport(ClientHttpTransport),
}

impl ClientTransport {
	// Note: Changed pub(crate) as it's internal detail now used via trait.
	pub(crate) async fn start(&mut self, transport_trx: TransportTrx) -> Result<()> {
		match self {
			ClientTransport::StdioTransport(transport) => transport.start(transport_trx).await?,
			ClientTransport::HttpTransport(transport) => transport.start(transport_trx).await?,
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

impl From<ClientHttpTransportConfig> for ClientTransport {
	fn from(config: ClientHttpTransportConfig) -> Self {
		let transport = ClientHttpTransport::from(config);
		transport.into()
	}
}

// endregion: --- Froms

// region:    --- IntoClientTransport Impls

/// Implement the public trait for the public config type.
impl IntoClientTransport for ClientStdioTransportConfig {
	fn into_client_transport(self) -> ClientTransport {
		// Reuse the From implementation defined above
		ClientTransport::from(self)
	}
}
impl client::into_client_transport::Sealed for ClientStdioTransportConfig {}

/// Implement the public trait for the public config type.
impl IntoClientTransport for ClientHttpTransportConfig {
	fn into_client_transport(self) -> ClientTransport {
		// Reuse the From implementation defined above
		ClientTransport::from(self)
	}
}
impl client::into_client_transport::Sealed for ClientHttpTransportConfig {}

/// Identity implementation for internal consistency/use.
impl IntoClientTransport for ClientTransport {
	fn into_client_transport(self) -> ClientTransport {
		self
	}
}

impl client::into_client_transport::Sealed for ClientTransport {}

// endregion: --- IntoClientTransport Impls
