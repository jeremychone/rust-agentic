use crate::mcp::{GenericMeta, IntoMcpNotification, McpNotification};
use serde::{Deserialize, Serialize};

// region:    --- PromptListChangedNotification

/// An optional notification from the server to the client, informing it that the list of prompts it offers has changed.
/// This may be issued by servers without any previous subscription from the client.
///
/// TS Ref: `PromptListChangedNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromptListChangedNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,
}

/// Builders
impl PromptListChangedNotificationParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpNotification for PromptListChangedNotificationParams {
	const METHOD: &'static str = "notifications/prompts/list_changed";
}

pub type PromptListChangedNotification = McpNotification<PromptListChangedNotificationParams>;

// endregion: --- PromptListChangedNotification

