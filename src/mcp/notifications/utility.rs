//! Utility-related notifications like logging and progress updates.

use crate::mcp::{GenericMeta, LoggingLevel, Notification, ProgressToken};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// region:    --- LoggingMessageNotification

/// Notification of a log message passed from server to client.
/// If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically.
///
/// TS Ref: `LoggingMessageNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingMessageNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The severity of this log message.
	pub level: LoggingLevel,

	/// An optional name of the logger issuing this message.
	pub logger: Option<String>,

	/// The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.
	pub data: Value,
}

impl LoggingMessageNotificationParams {
	pub const METHOD: &'static str = "notifications/message";
}

pub type LoggingMessageNotification = Notification<LoggingMessageNotificationParams>;

// endregion: --- LoggingMessageNotification

// region:    --- ProgressNotification

/// An out-of-band notification used to inform the receiver of a progress update for a long-running request.
///
/// TS Ref: `ProgressNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.
	pub progress_token: ProgressToken,

	/// The progress thus far. This should increase every time progress is made, even if the total is unknown.
	/// NOTE: For now, integer only
	pub progress: i64,

	/// Total number of items to process (or total progress required), if known.
	/// NOTE: For now, integer only
	pub total: Option<i64>,

	/// An optional message describing the current progress.
	pub message: Option<String>,
}

impl ProgressNotificationParams {
	pub const METHOD: &'static str = "notifications/progress";
}

pub type ProgressNotification = Notification<ProgressNotificationParams>;

// endregion: --- ProgressNotification
