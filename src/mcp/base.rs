use crate::mcp::ProgressToken;
use rpc_router::RpcId;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub const JSONRPC_VERSION: &str = "2.0";

/// Base structure for JSON-RPC Notifications.
/// Note: JSON-RPC 2.0 notifications do not have an 'id' field.
/// They MUST include a 'jsonrpc' field with value "2.0" and a 'method' field.
/// The 'params' field MAY be omitted.
/// This struct represents the common part after parsing/before serialization.
/// The actual 'method' is typically defined as a const on the params type.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification<P = Value> {
	/// The Method of this notification
	method: String,

	/// NOTE: TS `Notification` type makes params optional, matching here,
	///       but the spec says params MAY be omitted entirely, not just be null.
	///       Using Option<P> allows omitting it via serde `skip_serializing_if`.
	#[serde(skip_serializing_if = "Option::is_none")]
	params: Option<P>,
}

// region:    --- Request Base Constructs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<P = Value> {
	/// The json-rpc id
	id: RpcId,

	/// The Method of this notification
	method: String,

	/// The Params
	params: Option<P>,
}

/// Metadata attachable to a request's `_meta` field.
///
/// NOTE: For now, we have the Rust representation with progres_token and the extra.
///       We will add convenient accessors.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMeta {
	/// If specified, the caller is requesting out-of-band progress notifications.
	pub progress_token: Option<ProgressToken>,

	/// Allow arbitrary other metadata.
	#[serde(flatten)]
	pub extra: Map<String, Value>,
}

// endregion: --- Request Base Constructs

// region:    --- Common Types

/// Metadata attachable to a notification's `_meta` field.
///
/// NOTE: For now, we have the Rust representation with progres_token and the extra.
///       We will add convenient accessors.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericMeta {
	/// Allow arbitrary other metadata.
	#[serde(flatten)]
	pub inner: Map<String, Value>,
}

// endregion: --- Common Types
