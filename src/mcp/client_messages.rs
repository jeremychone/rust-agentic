//! Enums aggregating all possible messages sent from Client to Server.

use super::*; // Import all re-exported types from mcp module
use crate::rpc::{RpcNotification, RpcRequest, RpcResponse}; // Using renamed Rpc types
use serde::{Deserialize, Serialize};

// region:    --- Client Request

/// Represents the specific parameters for any Request the Client can send to the Server.
/// This enum is used as the `specific: T` part within `RequestParams<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
pub enum ClientRequestSpecificParams {
	#[serde(rename = "ping")]
	Ping(PingParams),

	#[serde(rename = "initialize")]
	Initialize(InitializeParams),

	#[serde(rename = "completion/complete")]
	Complete(CompleteParams),

	#[serde(rename = "logging/setLevel")]
	SetLevel(SetLevelParams),

	#[serde(rename = "prompts/get")]
	GetPrompt(GetPromptParams),

	#[serde(rename = "prompts/list")]
	ListPrompts(ListPromptsParams), // Use specific struct

	#[serde(rename = "resources/list")]
	ListResources(ListResourcesParams), // Use specific struct

	#[serde(rename = "resources/templates/list")]
	ListResourceTemplates(ListResourceTemplatesParams), // Use specific struct

	#[serde(rename = "resources/read")]
	ReadResource(ReadResourceParams),

	#[serde(rename = "resources/subscribe")]
	Subscribe(SubscribeParams),

	#[serde(rename = "resources/unsubscribe")]
	Unsubscribe(UnsubscribeParams),

	#[serde(rename = "tools/call")]
	CallTool(CallToolParams),

	#[serde(rename = "tools/list")]
	ListTools(ListToolsParams), // Use specific struct
}

/// Type alias for a full Client RPC Request using the MCP `RequestParams` wrapper
/// and the enum representing all possible specific parameters.
pub type ClientRpcRequest = RpcRequest<RequestParams<ClientRequestSpecificParams>>;

// endregion: --- Client Request

// region:    --- Client Notification

/// Represents the specific parameters for any Notification the Client can send to the Server.
/// This enum is used as the `specific: T` part within `NotificationParams<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
pub enum ClientNotificationSpecificParams {
	#[serde(rename = "notifications/cancelled")]
	Cancelled(CancelledParams),
	#[serde(rename = "notifications/progress")]
	Progress(ProgressParams),
	#[serde(rename = "notifications/initialized")]
	Initialized(InitializedParams),
	#[serde(rename = "notifications/roots/list_changed")]
	RootsListChanged(RootsListChangedParams),
}

/// Type alias for a full Client RPC Notification using the MCP `NotificationParams` wrapper
/// and the enum representing all possible specific parameters.
pub type ClientRpcNotification = RpcNotification<NotificationParams<ClientNotificationSpecificParams>>;

// endregion: --- Client Notification

// region:    --- Client Result

/// Represents the specific result data for any successful Response the Client can send to the Server.
/// This enum is used as the `specific: T` part within `ResultData<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)] // Result type is determined by context (the original request)
pub enum ClientResultSpecificData {
	// Responses to Server Requests
	CreateMessage(CreateMessageResultData),
	ListRoots(ListRootsResultData),
	// Catch-all for methods that return an empty success response (like Ping)
	Empty(EmptyResultData),
}

/// Type alias for a full Client RPC Response using the MCP `ResultData` wrapper
/// and the enum representing all possible specific result data structures.
pub type ClientRpcResponse = RpcResponse<ResultData<ClientResultSpecificData>>;

// endregion: --- Client Result
