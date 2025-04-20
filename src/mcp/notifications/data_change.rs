//! Notifications related to changes in server data listings or specific items.

use crate::mcp::{GenericMeta, Notification};
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
} // No parameters

impl ResourceListChangedNotificationParams {
	pub const METHOD: &'static str = "notifications/resources/list_changed";
}

pub type ResourceListChangedNotification = Notification<ResourceListChangedNotificationParams>;

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

impl ResourceUpdatedNotificationParams {
	pub const METHOD: &'static str = "notifications/resources/updated";
}

pub type ResourceUpdatedNotification = Notification<ResourceUpdatedNotificationParams>;

// endregion: --- ResourceUpdatedNotification

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
} // No parameters

impl PromptListChangedNotificationParams {
	pub const METHOD: &'static str = "notifications/prompts/list_changed";
}

pub type PromptListChangedNotification = Notification<PromptListChangedNotificationParams>;

// endregion: --- PromptListChangedNotification

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

impl ToolListChangedNotificationParams {
	pub const METHOD: &'static str = "notifications/tools/list_changed";
}

pub type ToolListChangedNotification = Notification<ToolListChangedNotificationParams>;

// endregion: --- ToolListChangedNotification

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
} // No parameters

impl RootsListChangedNotificationParams {
	pub const METHOD: &'static str = "notifications/roots/list_changed";
}

pub type RootsListChangedNotification = Notification<RootsListChangedNotificationParams>;

// endregion: --- RootsListChangedNotification
