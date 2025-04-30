//! Utility-related notifications like logging and progress updates.

use crate::mcp::{GenericMeta, IntoMcpNotification, LoggingLevel, McpNotification, ProgressToken};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// region:    --- LogNotification

/// Optional notification from the server to the client for logging purposes.
///
/// TS Ref: `LogNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// Logging level. Default should be assumed by the client if not provided (e.g., Info).
	pub level: Option<LoggingLevel>,

	/// The log message.
	pub message: String,

	/// Optional additional structured data.
	pub data: Option<Value>,
}

/// Builders
impl LogNotificationParams {
	pub fn new(message: impl Into<String>) -> Self {
		Self {
			meta: None,
			level: None,
			message: message.into(),
			data: None,
		}
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_level(mut self, level: LoggingLevel) -> Self {
		self.level = Some(level);
		self
	}

	pub fn with_data(mut self, data: Value) -> Self {
		self.data = Some(data);
		self
	}
}

impl IntoMcpNotification for LogNotificationParams {
	const METHOD: &'static str = "notifications/log";
}

pub type LogNotification = McpNotification<LogNotificationParams>;

// endregion: --- LogNotification

// region:    --- ProgressNotification

/// Optional notification from the server to the client about the progress of a long-running operation.
/// Requires a `progressToken` previously provided by the client in the corresponding request.
///
/// TS Ref: `ProgressNotification`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressNotificationParams {
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The token identifying the specific operation this progress update pertains to.
	pub progress_token: ProgressToken,

	/// The progress value/details. The structure of this value is defined by the specific method
	/// that initiated the operation being tracked.
	pub value: Value,
}

/// Builders
impl ProgressNotificationParams {
	pub fn new(progress_token: impl Into<ProgressToken>, value: Value) -> Self {
		Self {
			meta: None,
			progress_token: progress_token.into(),
			value,
		}
	}

	pub fn with_meta(mut self, meta: GenericMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpNotification for ProgressNotificationParams {
	const METHOD: &'static str = "notifications/progress";
}

pub type ProgressNotification = McpNotification<ProgressNotificationParams>;

// endregion: --- ProgressNotification

