use crate::RpcId;
use crate::mcp::{Cursor, GenericMeta, ProgressToken}; // Added GenericMeta import
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub const JSONRPC_VERSION: &str = "2.0";

/// Describes the name and version of an MCP implementation.
/// Used in InitializeRequest and InitializeResult.
///
/// TS Ref: `Implementation`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Implementation {
	pub name: String,
	pub version: String,
}

/// Builders
impl Implementation {
	pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			version: version.into(),
		}
	}
}

/// Base structure for requests supporting pagination.
/// Specific request types embed these fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
	/// An opaque token representing the current pagination position.
	/// If provided, the server should return results starting after this cursor.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cursor: Option<Cursor>,
}

/// Builders
impl PaginationParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_cursor(mut self, cursor: impl Into<Cursor>) -> Self {
		self.cursor = Some(cursor.into());
		self
	}
}

/// A request to include context from one or more MCP servers (including the caller),
/// to be attached to the prompt.
///
/// TS Ref: `CreateMessageRequest.params.includeContext`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum IncludeContext {
	None,
	ThisServer,
	AllServers,
}

/// Optional annotations for the client. The client can use annotations to inform how objects are used or displayed
///
/// TS Ref: `Annotations`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
	/// Describes who the intended customer of this object or data is.
	/// It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).
	pub audience: Option<Vec<Role>>,

	/// Describes how important this data is for operating the server.
	/// A value of 1 means "most important," and indicates that the data is effectively required,
	/// while 0 means "least important," and indicates that the data is entirely optional.
	/// @minimum 0
	/// @maximum 1
	pub priority: Option<f64>,
}

/// Builders
impl Annotations {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_audience(mut self, audience: Vec<Role>) -> Self {
		self.audience = Some(audience);
		self
	}

	pub fn append_audience(mut self, role: Role) -> Self {
		self.audience.get_or_insert_with(Vec::new).push(role);
		self
	}

	pub fn with_priority(mut self, priority: f64) -> Self {
		self.priority = Some(priority);
		self
	}
}

/// The sender or recipient of messages and data in a conversation.
///
/// TS Ref: `Role`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	User,
	Assistant,
}

/// Represents an empty JSON-RPC result, potentially containing only metadata.
/// Used for requests like `ping`, `logging/setLevel`, `resources/subscribe`, `resources/unsubscribe`.
///
/// TS Ref: `EmptyResult`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,
}

/// Builders
impl EmptyResult {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}
