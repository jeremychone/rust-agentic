use crate::mcp::{GenericMeta, IntoMcpNotification, IntoMcpRequest, McpNotification, RequestMeta};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The severity of a log message.
///
/// These map to syslog message severities, as specified in RFC-5424:
/// https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

// region:    --- SetLevelRequest

/// A request from the client to the server, to enable or adjust logging.
///
/// TS Ref: `SetLevelRequest`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLevelParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The level of logging that the client wants to receive from the server.
	/// The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message.
	pub level: LoggingLevel,
}

impl IntoMcpRequest for SetLevelParams {
	const METHOD: &'static str = "logging/setLevel";
}

// Note: The result for SetLevelRequest is `EmptyResult`, which translates to a standard JSON-RPC success response
// with an empty `result` object (potentially containing only `_meta`).
// No specific struct is needed for `EmptyResult` itself beyond standard result handling.

// endregion: --- SetLevelRequest

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

impl IntoMcpNotification for LoggingMessageNotificationParams {
	const METHOD: &'static str = "notifications/message";
}

pub type LoggingMessageNotification = McpNotification<LoggingMessageNotificationParams>;

// endregion: --- LoggingMessageNotification
