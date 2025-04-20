//! Utility-related requests like logging control, autocompletion, and root listing.

use crate::mcp::{GenericMeta, LoggingLevel, RequestMeta, Root};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

// region:    --- SetLevelRequest

/// A request from the client to the server, to enable or adjust logging.
///
/// TS Ref: `SetLevelRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLevelParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The level of logging that the client wants to receive from the server.
	/// The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message.
	pub level: LoggingLevel,
}

impl SetLevelParams {
	pub const METHOD: &'static str = "logging/setLevel";
}

// Note: The result for SetLevelRequest is `EmptyResult`, which translates to a standard JSON-RPC success response
// with an empty `result` object (potentially containing only `_meta`).
// No specific struct is needed for `EmptyResult` itself beyond standard result handling.

// endregion: --- SetLevelRequest

// region:    --- ListRootsRequest

/// Sent from the server to request a list of root URIs from the client. Roots allow
/// servers to ask for specific directories or files to operate on.
///
/// TS Ref: `ListRootsRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsParams {
	/// Optional metadata (_meta) for the request.
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,
	// Note: ListRootsRequest has no specific parameters beyond the optional _meta inherited from Request.
}

impl ListRootsParams {
	pub const METHOD: &'static str = "roots/list";
}

/// The client's response to a roots/list request from the server.
/// This result contains an array of Root objects, each representing a root directory
/// or file that the server can operate on.
///
/// TS Ref: `ListRootsResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRootsResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The list of roots.
	pub roots: Vec<Root>,
}

// endregion: --- ListRootsRequest
