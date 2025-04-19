//! Base MCP types like constants, identifiers, roles, and common structures.

use serde::{Deserialize, Serialize};

pub const LATEST_PROTOCOL_VERSION: &str = "2025-03-26";

/// A progress token, used to associate progress notifications with the original request.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProgressToken {
	String(String),
	Number(i64),
}

/// An opaque token used to represent a cursor for pagination.
pub type Cursor = String;

/// Represents an empty result (success with no data). Used as the `specific` part in `ResultData`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EmptyResultData {}

/// The sender or recipient of messages and data in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
	User,
	Assistant,
}

/// Optional annotations for the client.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
	/// Describes who the intended customer of this object or data is.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audience: Option<Vec<Role>>,
	/// Describes how important this data is. 1 means most important, 0 least important.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub priority: Option<f64>, // Using f64 for 0.0 to 1.0 range
	                           // NOTE: Although not in the original types.rs, Annotations *could* have arbitrary extra fields.
	                           // If that's desired, add:
	                           // #[serde(flatten)]
	                           // pub extra: std::collections::HashMap<String, Value>,
}

/// Describes the name and version of an MCP implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Implementation {
	pub name: String,
	pub version: String,
}
