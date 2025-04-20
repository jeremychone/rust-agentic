//! Types related to the initialize request/response and initialized notification.

use super::{Implementation, Notification, NotificationParams, Request, RequestParams, ResultData}; // Use re-exports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// region:    --- Initialize Request

/// Specific parameters for the InitializeRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
	/// The latest MCP version the client supports.
	pub protocol_version: String,

	/// Capabilities the client supports.
	pub capabilities: ClientCapabilities,

	/// Information about the client implementation.
	pub client_info: Implementation,
}

/// Type alias for the full Initialize Request structure using the wrapper.
pub type InitializeRequest = RequestParams<InitializeParams>;

/// Associates the params/result structures with the Request trait.
impl Request for InitializeParams {
	const METHOD: &'static str = "initialize";
	type Params = Self;
	type Result = InitializeResultData;
}

// endregion: --- Initialize Request

// region:    --- Initialize Result

/// Specific result data for the Initialize response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResultData {
	/// The MCP version the server wants to use.
	pub protocol_version: String,

	/// Capabilities the server supports.
	pub capabilities: ServerCapabilities,

	/// Information about the server implementation.
	pub server_info: Implementation,

	/// Optional instructions describing how to use the server.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
}

/// Type alias for the full Initialize Result structure using the wrapper.
pub type InitializeResult = ResultData<InitializeResultData>;

// endregion: --- Initialize Result

// region:    --- Initialized Notification

/// Parameters for the InitializedNotification (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct InitializedParams {}

/// Type alias for the full Initialized Notification structure using the wrapper.
pub type InitializedNotification = NotificationParams<InitializedParams>;

/// Associates the params structure with the Notification trait.
impl Notification for InitializedParams {
	const METHOD: &'static str = "notifications/initialized";
	type Params = Self;
}

// endregion: --- Initialized Notification

// region:    --- Capabilities

/// Capabilities a client may support.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
	/// Experimental, non-standard capabilities.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub experimental: Option<HashMap<String, Value>>,

	/// Present if the client supports listing roots.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roots: Option<RootsClientCapabilities>,

	/// Present if the client supports sampling from an LLM. Using Value as it's just an empty object marker.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sampling: Option<Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootsClientCapabilities {
	/// Whether the client supports notifications for changes to the roots list.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub list_changed: Option<bool>,
}

/// Capabilities that a server may support.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
	/// Experimental, non-standard capabilities.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub experimental: Option<HashMap<String, Value>>,

	/// Present if the server supports sending log messages. Using Value as it's just an empty object marker.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logging: Option<Value>,

	/// Present if the server supports argument autocompletion. Using Value as it's just an empty object marker.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completions: Option<Value>,

	/// Present if the server offers any prompt templates.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompts: Option<PromptsServerCapabilities>,

	/// Present if the server offers any resources to read.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resources: Option<ResourcesServerCapabilities>,

	/// Present if the server offers any tools to call.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<ToolsServerCapabilities>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptsServerCapabilities {
	/// Whether this server supports notifications for changes to the prompt list.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesServerCapabilities {
	/// Whether this server supports subscribing to resource updates.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscribe: Option<bool>,

	/// Whether this server supports notifications for changes to the resource list.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub list_changed: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsServerCapabilities {
	/// Whether this server supports notifications for changes to the tool list.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub list_changed: Option<bool>,
}

// endregion: --- Capabilities
