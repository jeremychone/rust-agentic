use crate::mcp::{GenericMeta, IntoMcpNotification, McpNotification};
use serde::{Deserialize, Serialize};

// region:    --- ToolListChangedNotification

/// An optional notification from the server to the client, informing it that the list of tools it offers has changed.
/// This may be issued by servers without any previous subscription from the client.
///
/// TS Ref: `ToolListChangedNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolListChangedNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,
} // No parameters

impl IntoMcpNotification for ToolListChangedNotificationParams {
	const METHOD: &'static str = "notifications/tools/list_changed";
}

pub type ToolListChangedNotification = McpNotification<ToolListChangedNotificationParams>;

// endregion: --- ToolListChangedNotification
