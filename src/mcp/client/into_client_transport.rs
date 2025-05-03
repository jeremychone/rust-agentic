use super::transport::ClientTransport; // internal type

pub trait Sealed {}

/// A trait for types that can be converted into an internal `ClientTransport`.
///
/// This allows the `Client::connect` method to accept various configurations
/// or pre-configured transport types (like `ClientStdioTransportConfig`)
/// without exposing the internal `ClientTransport` enum directly in the signature.
pub trait IntoClientTransport: Sealed {
	/// Performs the conversion into a `ClientTransport`.
	fn into_client_transport(self) -> ClientTransport;
}
