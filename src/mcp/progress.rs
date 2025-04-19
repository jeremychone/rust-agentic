//! Types related to the notifications/progress notification.

use super::{Notification, NotificationParams, ProgressToken}; // Use re-exports
use serde::{Deserialize, Serialize};

/// Specific parameters for the ProgressNotification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressParams {
	/// The token associated with the original request.
	pub progress_token: ProgressToken,

	/// The progress thus far.
	pub progress: f64, // Use f64 to represent numeric progress

	/// Total number of items or total progress required, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total: Option<f64>,

	/// Optional message describing the current progress.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
}

/// Type alias for the full Progress Notification structure using the wrapper.
pub type ProgressNotification = NotificationParams<ProgressParams>;

/// Associates the params structure with the Notification trait.
impl Notification for ProgressParams {
	const METHOD: &'static str = "notifications/progress";
	type Params = Self;
}
