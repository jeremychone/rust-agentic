use crate::mcp::Annotations;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

/// Definition for a tool the client can call.
///
/// TS Ref: `Tool`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
	/// The name of the tool.
	pub name: String,

	/// A human-readable description of the tool.
	pub description: Option<String>,

	/// A JSON Schema object defining the expected parameters for the tool.
	pub input_schema: ToolInputSchema,

	/// Optional additional tool information.
	pub annotations: Option<ToolAnnotations>,
}

/// The input schema for a tool.
///
/// TS Ref: `Tool.inputSchema`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInputSchema {
	/// Must be "object" for tool input schemas
	#[serde(rename = "type")]
	pub schema_type: String,

	/// Properties of the input schema
	pub properties: Option<Value>,

	/// Required properties in the input schema
	pub required: Option<Vec<String>>,
}

/// Additional properties describing a Tool to clients.
///
/// NOTE: all properties in ToolAnnotations are **hints**.
/// They are not guaranteed to provide a faithful description of
/// tool behavior (including descriptive properties like `title`).
///
/// TS Ref: `ToolAnnotations`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
	/// A human-readable title for the tool.
	pub title: Option<String>,

	/// If true, the tool does not modify its environment.
	/// Default: false
	pub read_only_hint: Option<bool>,

	/// If true, the tool may perform destructive updates to its environment.
	/// If false, the tool performs only additive updates.
	/// (This property is meaningful only when `readOnlyHint == false`)
	/// Default: true
	pub destructive_hint: Option<bool>,

	/// If true, calling the tool repeatedly with the same arguments
	/// will have no additional effect on the its environment.
	/// (This property is meaningful only when `readOnlyHint == false`)
	/// Default: false
	pub idempotent_hint: Option<bool>,

	/// If true, this tool may interact with an "open world" of external
	/// entities. If false, the tool's domain of interaction is closed.
	/// For example, the world of a web search tool is open, whereas that
	/// of a memory tool is not.
	/// Default: true
	pub open_world_hint: Option<bool>,
}
