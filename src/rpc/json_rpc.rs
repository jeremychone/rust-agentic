use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const JSONRPC_VERSION: &str = "2.0";

// region:    --- Error Codes

pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

// endregion: --- Error Codes

// region:    --- Core Types

/// A uniquely identifying ID for a request in JSON-RPC.
/// Can be a String, Number, or Null (Null is discouraged but allowed by spec).
/// We represent Number as i64 for simplicity, acknowledging JSON numbers can be larger.
/// Null ID for a request is usually considered invalid by implementations, but we include
/// it for spec completeness in parsing, though sending Null ID requests is problematic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
	String(String),
	Number(i64),
	// Null, // Discouraged, can cause issues with response matching. Omitted for robustness.
}

/// A request object that expects a response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcRequest<P = Value> {
	pub jsonrpc: String, // Should always be "2.0"

	pub id: RequestId,

	pub method: String,

	/// Parameters for the method. Can be structured (object/array) or omitted.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<P>,
}

/// A notification object which does not expect a response.
/// ID field MUST NOT exist for notifications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcNotification<P = Value> {
	pub jsonrpc: String, // Should always be "2.0"

	pub method: String,

	/// Parameters for the method. Can be structured (object/array) or omitted.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<P>,
}

/// A successful (non-error) response object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcResponse<R = Value> {
	pub jsonrpc: String, // Should always be "2.0"

	pub id: RequestId, // Must match the request ID it's responding to.

	/// The result of the method execution. Required on success.
	pub result: R,
}

/// The error object included in an RpcError response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcErrorObject {
	/// A Number that indicates the error type that occurred.
	pub code: i32,

	/// A String providing a short description of the error.
	pub message: String,

	/// A Primitive or Structured value containing additional information about the error.
	/// May be omitted.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Value>,
}

/// An error response object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcError {
	pub jsonrpc: String, // Should always be "2.0"

	/// Must match the request ID it's responding to.
	/// If the error occurred before the ID could be determined (e.g., Parse error), it should be Null.
	/// We use Option<RequestId> to represent this possibility.
	pub id: Option<RequestId>, // Changed to Option<RequestId> to handle cases like parse errors where ID might be null or unknown.

	/// The error object containing details about the error.
	pub error: RpcErrorObject,
}

// endregion: --- Core Types

// region:    --- Aggregation Types

/// Represents either a success or an error response for a single request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcSingleResponse<R = Value> {
	Success(RpcResponse<R>),
	Error(RpcError),
}

/// Represents either a Request or a Notification. Used within Batch requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcSingleRequest<P = Value> {
	Request(RpcRequest<P>),
	Notification(RpcNotification<P>),
}

/// A JSON-RPC batch request, containing an array of Request and/or Notification objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RpcBatchRequest<P = Value>(pub Vec<RpcSingleRequest<P>>);

/// A JSON-RPC batch response, containing an array of Response objects corresponding to the requests.
/// The response array might be smaller than the request array if notifications were included.
/// The order of responses does not necessarily match the order of requests.
/// An empty array is returned if the batch request only contained notifications.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RpcBatchResponse<R = Value>(pub Vec<RpcSingleResponse<R>>);

/// Represents any valid JSON-RPC message (single or batch, request, response, notification, error).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcMessage<P = Value, R = Value> {
	// Order might matter for deserialization: Batch structures first.
	BatchRequest(RpcBatchRequest<P>),
	BatchResponse(RpcBatchResponse<R>),
	// Single messages (order between these less critical, but request/response might be more common).
	Request(RpcRequest<P>),
	Response(RpcResponse<R>),
	Notification(RpcNotification<P>),
	Error(RpcError), // Single Error object (can occur outside batch response context, though less common)
}

// endregion: --- Aggregation Types
