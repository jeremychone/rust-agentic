//! Enums aggregating all possible messages sent from Server to Client.

use super::*; // Import all re-exported types from mcp module
use crate::rpc::{RpcNotification, RpcRequest, RpcResponse}; // Using renamed Rpc types
use serde::{Deserialize, Serialize};

// region:    --- Server Request

/// Represents the specific parameters for any Request the Server can send to the Client.
/// This enum is used as the `specific: T` part within `RequestParams<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
pub enum ServerRequestSpecificParams {
	#[serde(rename = "ping")]
	Ping(PingParams),

	#[serde(rename = "sampling/createMessage")]
	CreateMessage(CreateMessageParams),

	#[serde(rename = "roots/list")]
	ListRoots(ListRootsParams),
}

/// Type alias for a full Server RPC Request using the MCP `RequestParams` wrapper
/// and the enum representing all possible specific parameters.
pub type ServerRpcRequest = RpcRequest<RequestParams<ServerRequestSpecificParams>>;

// endregion: --- Server Request

// region:    --- Server Notification

/// Represents the specific parameters for any Notification the Server can send to the Client.
/// This enum is used as the `specific: T` part within `NotificationParams<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
pub enum ServerNotificationSpecificParams {
	#[serde(rename = "notifications/cancelled")]
	Cancelled(CancelledParams),

	#[serde(rename = "notifications/progress")]
	Progress(ProgressParams),

	#[serde(rename = "notifications/message")] // Logging message
	Message(LoggingMessageParams),

	#[serde(rename = "notifications/resources/updated")]
	ResourceUpdated(ResourceUpdatedParams),

	#[serde(rename = "notifications/resources/list_changed")]
	ResourceListChanged(ResourceListChangedParams),

	#[serde(rename = "notifications/tools/list_changed")]
	ToolListChanged(ToolListChangedParams),

	#[serde(rename = "notifications/prompts/list_changed")]
	PromptListChanged(PromptListChangedParams),
}

/// Type alias for the full Server RPC Notification using the MCP `NotificationParams` wrapper
/// and the enum representing all possible specific parameters.
pub type ServerRpcNotification = RpcNotification<NotificationParams<ServerNotificationSpecificParams>>;

// endregion: --- Server Notification

// region:    --- Server Result

/// Represents the specific result data for any successful Response the Server can send to the Client.
/// This enum is used as the `specific: T` part within `ResultData<T>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)] // Result type is determined by context (the original request)
pub enum ServerResultSpecificData {
	// Responses to Client Requests
	Initialize(InitializeResultData),

	Complete(CompleteResultData),

	GetPrompt(GetPromptResultData),

	ListPrompts(ListPromptsResultData), // Use specific struct

	ListResources(ListResourcesResultData), // Use specific struct

	ListResourceTemplates(ListResourceTemplatesResultData), // Use specific struct

	ReadResource(ReadResourceResultData),

	CallTool(CallToolResultData),

	ListTools(ListToolsResultData), // Use specific struct

	// Catch-all for methods that return an empty success response
	// (like Ping, SetLevel, Subscribe, Unsubscribe)
	Empty(EmptyResultData),
}

/// Type alias for a full Server RPC Response using the MCP `ResultData` wrapper
/// and the enum representing all possible specific result data structures.
pub type ServerRpcResponse = RpcResponse<ResultData<ServerResultSpecificData>>;

// endregion: --- Server Result
