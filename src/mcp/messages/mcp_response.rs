use crate::mcp::{
	CallToolResult, CompleteResult, CreateMessageResult, EmptyResult, GetPromptResult, InitializeResult,
	ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult, ListRootsResult, ListToolsResult,
	ReadResourceResult,
};
use crate::mcp::{Error, Result};
use rpc_router::{RpcError, RpcId, RpcResponse, RpcSuccessResponse};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use serde_json::Value;

/// Represents a successful MCP response.
///
/// This struct holds the structured `Result` type `R` for a specific MCP method
/// or `Value` if the type is not known beforehand. It corresponds to the `result`
/// field of a successful JSON-RPC 2.0 response.
#[derive(Debug, Clone)]
pub struct McpResponse<R = Value> {
	pub id: RpcId,
	pub result: R,
}

impl<P: Serialize> McpResponse<P> {
	pub fn stringify(&self) -> Result<String> {
		serde_json::to_string(&self).map_err(Error::custom_from_err)
	}
	pub fn stringify_pretty(&self) -> Result<String> {
		serde_json::to_string_pretty(&self).map_err(Error::custom_from_err)
	}
}

// region:    --- Custom De/Serialization

impl<R> Serialize for McpResponse<R>
where
	R: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Serialize the inner result to a Value
		let result_value = serde_json::to_value(&self.result).map_err(serde::ser::Error::custom)?;

		// Construct the RpcSuccessResponse
		let success_response = RpcSuccessResponse {
			id: self.id.clone(),
			result: result_value,
		};

		// Wrap in RpcResponse::Success and serialize
		let rpc_response = RpcResponse::Success(success_response);
		rpc_response.serialize(serializer)
	}
}

impl<'de, R> Deserialize<'de> for McpResponse<R>
where
	R: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Deserialize into the RpcResponse enum first
		let rpc_response = RpcResponse::deserialize(deserializer)?;

		// Ensure it's a success response
		match rpc_response {
			RpcResponse::Success(success) => {
				// Deserialize the inner result Value into R
				let result = R::deserialize(success.result).map_err(DeError::custom)?;

				Ok(McpResponse { id: success.id, result })
			}
			RpcResponse::Error(err) => {
				// McpResponse only represents success cases.
				// Deserializing an RpcErrorResponse into McpResponse is an error.
				Err(DeError::custom(format!(
					"Expected a success response, but got an error response: id={}, code={}, message='{}'",
					err.id, err.error.code, err.error.message
				)))
			}
		}
	}
}

// endregion: --- Custom De/Serialization
