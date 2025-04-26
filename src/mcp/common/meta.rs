use crate::mcp::ProgressToken;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
