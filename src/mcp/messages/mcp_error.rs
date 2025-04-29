use crate::mcp::{Error, Result};
use rpc_router::{RpcError, RpcErrorResponse, RpcId, RpcResponse};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

/// Represents an MCP Error response, corresponding to the JSON-RPC 2.0 Error Object.
///
/// This struct holds the structured `RpcError` type for a specific MCP error response.
#[derive(Debug, Clone)]
pub struct McpError {
	pub id: RpcId,
	pub error: RpcError,
}

impl McpError {
	pub fn stringify(&self) -> Result<String> {
		serde_json::to_string(&self).map_err(Error::custom_from_err)
	}
}

// region:    --- Custom De/Serialization

impl Serialize for McpError {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Construct the RpcErrorResponse
		let error_response = RpcErrorResponse {
			id: self.id.clone(),
			error: self.error.clone(), // RpcError should be cloneable
		};

		// Wrap in RpcResponse::Error and serialize
		let rpc_response = RpcResponse::Error(error_response);
		rpc_response.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for McpError {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Deserialize into the RpcResponse enum first
		let rpc_response = RpcResponse::deserialize(deserializer)?;

		// Ensure it's an error response
		match rpc_response {
			RpcResponse::Error(err) => Ok(McpError {
				id: err.id,
				error: err.error,
			}),
			RpcResponse::Success(success) => {
				// McpError only represents error cases.
				// Deserializing an RpcSuccessResponse into McpError is an error.
				Err(DeError::custom(format!(
					"Expected an error response, but got a success response: id={}",
					success.id
				)))
			}
		}
	}
}

// endregion: --- Custom De/Serialization
