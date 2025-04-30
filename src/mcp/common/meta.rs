use crate::mcp::ProgressToken;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// region:    --- GenericMeta

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
	pub inner: Option<Map<String, Value>>,
}

impl GenericMeta {
	pub fn append(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
		self.inner.get_or_insert_with(Map::new).insert(name.into(), value.into());
		self
	}
}

// endregion: --- GenericMeta

// region:    --- RequestMeta

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
	pub extra: Option<Map<String, Value>>,
}

/// Builders
impl RequestMeta {
	pub fn with_progress_token(mut self, token: impl Into<ProgressToken>) -> Self {
		self.progress_token = Some(token.into());
		self
	}

	pub fn with_extra(mut self, extra: Map<String, Value>) -> Self {
		self.extra = Some(extra);
		self
	}

	pub fn append(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
		self.extra.get_or_insert_with(Map::new).insert(name.into(), value.into());
		self
	}
}

// endregion: --- RequestMeta
