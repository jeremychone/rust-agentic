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

/// Builders
impl Tool {
	pub fn new(name: impl Into<String>, input_schema: ToolInputSchema) -> Self {
		Self {
			name: name.into(),
			description: None,
			input_schema,
			annotations: None,
		}
	}

	pub fn with_description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn with_annotations(mut self, annotations: ToolAnnotations) -> Self {
		self.annotations = Some(annotations);
		self
	}
}

/// The input schema for a tool.
///
/// TS Ref: `Tool.inputSchema`
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInputSchema {
	/// Must be "object" for tool input schemas
	#[serde(rename = "type")]
	pub schema_type: String, // Typically "object"

	/// Properties of the input schema
	pub properties: Option<Value>, // Usually a JSON Object like Map<String, JsonSchema>

	/// Required properties in the input schema
	pub required: Option<Vec<String>>,
}

/// Builders
impl ToolInputSchema {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	/// Note: Usually a Map<String, JsonSchema> represented as serde_json::Value
	pub fn with_properties(mut self, properties: Value) -> Self {
		self.properties = Some(properties);
		self
	}

	pub fn append_required(mut self, required_prop: impl Into<String>) -> Self {
		self.required.get_or_insert_with(Vec::new).push(required_prop.into());
		self
	}
}

/// Additional properties describing a Tool to clients.
///
/// NOTE: all properties in ToolAnnotations are **hints**.
/// They are not guaranteed to provide a faithful description of
/// tool behavior (including descriptive properties like `title`).
///
/// TS Ref: `ToolAnnotations`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Builders
impl ToolAnnotations {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn with_read_only_hint(mut self, read_only: bool) -> Self {
		self.read_only_hint = Some(read_only);
		self
	}

	pub fn with_destructive_hint(mut self, destructive: bool) -> Self {
		self.destructive_hint = Some(destructive);
		self
	}

	pub fn with_idempotent_hint(mut self, idempotent: bool) -> Self {
		self.idempotent_hint = Some(idempotent);
		self
	}

	pub fn with_open_world_hint(mut self, open_world: bool) -> Self {
		self.open_world_hint = Some(open_world);
		self
	}
}
