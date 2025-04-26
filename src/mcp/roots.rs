//! Utility-related requests like logging control, autocompletion, and root listing.

use crate::mcp::{GenericMeta, IntoMcpRequest, LoggingLevel, RequestMeta, Root};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

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

impl IntoMcpRequest for ListRootsParams {
	const METHOD: &'static str = "roots/list";
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
