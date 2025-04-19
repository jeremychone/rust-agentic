//! MCP message parameter and result wrappers adding the `_meta` field.

use super::base::ProgressToken;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// region:    --- MCP Wrappers

/// Base structure for MCP Request parameters, adding the optional _meta field.
/// The specific parameters `T` are flattened into this structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestParams<T> {
	#[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
	pub meta: Option<RequestMeta>,
	#[serde(flatten)]
	pub specific: T,
}

/// Metadata attachable to a request's `_meta` field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMeta {
	/// If specified, the caller is requesting out-of-band progress notifications.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub progress_token: Option<ProgressToken>,
	/// Allow arbitrary other metadata.
	#[serde(flatten)]
	pub extra: HashMap<String, Value>,
}

/// Base structure for MCP Notification parameters, adding the optional _meta field.
/// The specific parameters `T` are flattened into this structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationParams<T> {
	#[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
	pub meta: Option<NotificationMeta>, // Using a dedicated struct for Notification meta
	#[serde(flatten)]
	pub specific: T,
}

/// Metadata attachable to a notification's `_meta` field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationMeta {
	/// Allow arbitrary metadata.
	#[serde(flatten)]
	pub extra: HashMap<String, Value>,
}

/// Base structure for MCP Result data, adding the optional _meta field.
/// The specific result data `T` are flattened into this structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultData<T> {
	#[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
	pub meta: Option<ResultMeta>, // Using a dedicated struct for Result meta
	#[serde(flatten)]
	pub specific: T,
}

/// Metadata attachable to a result's `_meta` field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResultMeta {
	/// Allow arbitrary metadata.
	#[serde(flatten)]
	pub extra: HashMap<String, Value>,
}

// endregion: --- MCP Wrappers
