use serde::{Deserialize, Serialize};

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

/// A progress token, used to associate progress notifications with the original request.
///
/// TS Ref: `ProgressToken`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ProgressToken {
	String(String),
	Number(i64),
}

pub type Cursor = String;
