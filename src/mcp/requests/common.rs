use crate::mcp::Cursor;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

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
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// The sender or recipient of messages and data in a conversation.
///
/// TS Ref: `Role`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	User,
	Assistant,
}
