use crate::mcp::{GenericMeta, IntoMcpNotification, McpNotification};
use derive_more::From;
use serde::{Deserialize, Serialize};

/// A progress token, used to associate progress notifications with the original request.
///
/// TS Ref: `ProgressToken`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, From)]
#[serde(untagged)]
pub enum ProgressToken {
	#[from(String, &str, &String)]
	String(String),
	#[from(i64, i32)]
	Number(i64),
}

pub type Cursor = String;

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

impl IntoMcpNotification for ProgressNotificationParams {
	const METHOD: &'static str = "notifications/progress";
}

pub type ProgressNotification = McpNotification<ProgressNotificationParams>;

// endregion: --- ProgressNotification
