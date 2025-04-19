//! Types related to logging configuration and messages.

use super::{EmptyResultData, Notification, NotificationParams, Request, RequestParams}; // Use re-exports
use serde::{Deserialize, Serialize};
use serde_json::Value;

// region:    --- Set Level Request

/// Specific parameters for the SetLevelRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLevelParams {
	/// The desired logging level.
	pub level: LoggingLevel,
}

/// Type alias for the full SetLevel Request structure using the wrapper.
pub type SetLevelRequest = RequestParams<SetLevelParams>;

/// Associates the params/result structures with the Request trait.
impl Request for SetLevelParams {
	const METHOD: &'static str = "logging/setLevel";
	type Params = Self;
	type Result = EmptyResultData; // Successful level set returns no data
}

// endregion: --- Set Level Request

// region:    --- Logging Message Notification

/// Specific parameters for the LoggingMessageNotification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingMessageParams {
	/// Severity of the log message.
	pub level: LoggingLevel,
	/// Optional name of the logger.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logger: Option<String>,
	/// The data to be logged (string, object, etc.).
	pub data: Value,
}

/// Type alias for the full Logging Message Notification structure using the wrapper.
pub type LoggingMessageNotification = NotificationParams<LoggingMessageParams>;

/// Associates the params structure with the Notification trait.
impl Notification for LoggingMessageParams {
	const METHOD: &'static str = "notifications/message";
	type Params = Self;
}

// endregion: --- Logging Message Notification

// region:    --- Logging Level Enum

/// The severity of a log message (maps to syslog severities).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
	Debug,
	Info,
	Notice,
	Warning,
	Error,
	Critical,
	Alert,
	Emergency,
}

// endregion: --- Logging Level Enum
