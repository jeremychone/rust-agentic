use crate::RpcId;
use crate::mcp::{GenericMeta, IntoMcpNotification, McpNotification};
use serde::{Deserialize, Serialize};

// region:    --- CancelledNotification

/// This notification can be sent by either side to indicate that it is cancelling a previously-issued request.
///
/// The request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.
///
/// This notification indicates that the result will be unused, so any associated processing SHOULD cease.
///
/// A client MUST NOT attempt to cancel its `initialize` request.
///
/// TS Ref: `CancelledNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelledNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The ID of the request to cancel.
	///
	/// NOTE: This is not the rpc_id of this Notification (as MCP notifications do not have )
	///
	/// This MUST correspond to the ID of a request previously issued in the same direction.
	pub request_id: RpcId,

	/// An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.
	pub reason: Option<String>,
}

impl IntoMcpNotification for CancelledNotificationParams {
	const METHOD: &'static str = "notifications/cancelled";
}

pub type CancelledNotification = McpNotification<CancelledNotificationParams>;

// endregion: --- CancelledNotification

// region:    --- InitializedNotification

/// This notification is sent from the client to the server after initialization has finished.
///
/// TS Ref: `InitializedNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InitializedNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,
} // No parameters

impl IntoMcpNotification for InitializedNotificationParams {
	const METHOD: &'static str = "notifications/initialized";
}

pub type InitializedNotification = McpNotification<InitializedNotificationParams>;

// endregion: --- InitializedNotification
