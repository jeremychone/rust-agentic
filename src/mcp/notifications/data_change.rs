//! Notifications related to changes in data listings or specific items (e.g., roots).

use crate::mcp::{GenericMeta, IntoMcpNotification, McpNotification}; // Adjusted imports if needed
use serde::{Deserialize, Serialize};

// Note: Resource-related notifications moved to src/mcp/resources/notifications.rs

// region:    --- RootsListChangedNotification

/// A notification from the client to the server, informing it that the list of roots has changed.
/// This notification should be sent whenever the client adds, removes, or modifies any root.
/// The server should then request an updated list of roots using the ListRootsRequest.
///
/// TS Ref: `RootsListChangedNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RootsListChangedNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,
}

/// Builders
impl RootsListChangedNotificationParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets the metadata for the notification.
	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpNotification for RootsListChangedNotificationParams {
	const METHOD: &'static str = "notifications/roots/list_changed";
}

pub type RootsListChangedNotification = McpNotification<RootsListChangedNotificationParams>;

// endregion: --- RootsListChangedNotification
