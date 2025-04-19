//! Types related to the notifications/cancelled notification.

use super::{Notification, NotificationParams}; // Use re-exports from super
use crate::rpc::RequestId;
use serde::{Deserialize, Serialize};

/// Specific parameters for the CancelledNotification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelledParams {
	/// The ID of the request to cancel.
	pub request_id: RequestId,
	/// Optional reason for the cancellation.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String>,
}

/// Type alias for the full Cancelled Notification structure using the wrapper.
pub type CancelledNotification = NotificationParams<CancelledParams>;

/// Associates the params structure with the Notification trait.
impl Notification for CancelledParams {
	const METHOD: &'static str = "notifications/cancelled";
	type Params = Self;
}
