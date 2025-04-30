//! Notifications related to changes in server resources.

use crate::mcp::{GenericMeta, IntoMcpNotification, McpNotification}; // Adjusted imports if needed
use serde::{Deserialize, Serialize};

// region:    --- ResourceListChangedNotification

/// An optional notification from the server to the client, informing it that the list of resources it can read from has changed.
/// This may be issued by servers without any previous subscription from the client.
///
/// TS Ref: `ResourceListChangedNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceListChangedNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,
}

/// Builders
impl ResourceListChangedNotificationParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpNotification for ResourceListChangedNotificationParams {
	const METHOD: &'static str = "notifications/resources/list_changed";
}

pub type ResourceListChangedNotification = McpNotification<ResourceListChangedNotificationParams>;

// endregion: --- ResourceListChangedNotification

// region:    --- ResourceUpdatedNotification

/// A notification from the server to the client, informing it that a resource has changed and may need to be read again.
/// This should only be sent if the client previously sent a resources/subscribe request.
///
/// TS Ref: `ResourceUpdatedNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUpdatedNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.
	/// Assuming URI is represented as a String. Consider using a dedicated URI type if needed.
	/// `@format uri` from schema
	pub uri: String,
}

/// Builders
impl ResourceUpdatedNotificationParams {
	pub fn new(uri: impl Into<String>) -> Self {
		Self {
			meta: None,
			uri: uri.into(),
		}
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpNotification for ResourceUpdatedNotificationParams {
	const METHOD: &'static str = "notifications/resources/updated";
}

pub type ResourceUpdatedNotification = McpNotification<ResourceUpdatedNotificationParams>;

// endregion: --- ResourceUpdatedNotification
