//! Types related to tool listing and execution.

use super::{
	AudioContent, EmbeddedResource, ImageContent, Notification, NotificationParams, PaginatedParams,
	PaginatedResultData, Request, RequestParams, ResultData, TextContent,
}; // Use re-exports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// region:    --- List Tools

/// Specific parameters for listing tools, including pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListToolsParams {
	#[serde(flatten)]
	pub pagination: PaginatedParams,
}

/// Type alias for the full ListTools Request structure using the wrapper.
pub type ListToolsRequest = RequestParams<ListToolsParams>;

/// Specific result data for listing tools.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListToolsResultData {
	#[serde(flatten)]
	pub pagination: PaginatedResultData,
	pub tools: Vec<Tool>,
}

/// Type alias for the full ListTools Result structure using the wrapper.
pub type ListToolsResult = ResultData<ListToolsResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for ListToolsParams {
	const METHOD: &'static str = "tools/list";
	type Params = Self;
	type Result = ListToolsResultData;
}

// endregion: --- List Tools

// region:    --- Call Tool

/// Specific parameters for invoking a tool.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallToolParams {
	/// The name of the tool to call.
	pub name: String,
	/// Arguments for the tool.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<HashMap<String, Value>>,
}

/// Type alias for the full CallTool Request structure using the wrapper.
pub type CallToolRequest = RequestParams<CallToolParams>;

/// Specific result data for a tool call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToolResultData {
	pub content: Vec<ToolResultContent>,
	/// Whether the tool call ended in an error. Defaults to false.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_error: Option<bool>,
}

/// Type alias for the full CallTool Result structure using the wrapper.
pub type CallToolResult = ResultData<CallToolResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for CallToolParams {
	const METHOD: &'static str = "tools/call";
	type Params = Self;
	type Result = CallToolResultData;
}

/// Content types allowed within a CallToolResult.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")] // Rely on enum tag for type
pub enum ToolResultContent {
	Text(TextContent),
	Image(ImageContent),
	Audio(AudioContent),
	Resource(EmbeddedResource),
}

// Implement Default for ToolResultContent (optional, defaults to Text)
impl Default for ToolResultContent {
	fn default() -> Self {
		ToolResultContent::Text(Default::default())
	}
}

// endregion: --- Call Tool

// region:    --- Tool List Changed Notification

/// Parameters for the ToolListChangedNotification (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ToolListChangedParams {}

/// Type alias for the full ToolListChanged Notification structure using the wrapper.
pub type ToolListChangedNotification = NotificationParams<ToolListChangedParams>;

/// Associates the params structure with the Notification trait.
impl Notification for ToolListChangedParams {
	const METHOD: &'static str = "notifications/tools/list_changed";
	type Params = Self;
}

// endregion: --- Tool List Changed Notification

// region:    --- Tool Definitions

/// Definition for a tool the client can call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
	/// The name of the tool.
	pub name: String,
	/// Human-readable description.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// JSON Schema object defining expected parameters.
	pub input_schema: ToolInputSchema,
	/// Optional additional tool information.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<ToolAnnotations>,
}

/// Specific structure for the tool input schema based on the definition.
/// It must be a JSON Schema object (`type: "object"`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolInputSchema {
	#[serde(rename = "type")]
	pub schema_type: String, // Should always be "object"
	/// Definition of properties within the object schema. Value represents arbitrary JSON Schema properties.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub properties: Option<HashMap<String, Value>>,
	/// List of required property names.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required: Option<Vec<String>>,
}

// Default ensures schema_type is "object"
impl Default for ToolInputSchema {
	fn default() -> Self {
		Self {
			schema_type: "object".to_string(),
			properties: None,
			required: None,
		}
	}
}

/// Optional additional tool information (hints).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
	/// Human-readable title.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/// Hint: Tool does not modify its environment. (Default: false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub read_only_hint: Option<bool>,
	/// Hint: Tool may perform destructive updates. (Default: true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub destructive_hint: Option<bool>,
	/// Hint: Tool is idempotent. (Default: false)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub idempotent_hint: Option<bool>,
	/// Hint: Tool interacts with an open world. (Default: true)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub open_world_hint: Option<bool>,
}

// endregion: --- Tool Definitions
