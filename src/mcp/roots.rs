//! Types related to workspace roots.

use super::{Notification, NotificationParams, Request, RequestParams, ResultData}; // Use re-exports
use serde::{Deserialize, Serialize};

// region:    --- List Roots Request

/// Specific parameters for the ListRootsRequest (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListRootsParams {}

/// Type alias for the full ListRoots Request structure using the wrapper.
pub type ListRootsRequest = RequestParams<ListRootsParams>;

/// Associates the params/result structures with the Request trait.
impl Request for ListRootsParams {
	const METHOD: &'static str = "roots/list";
	type Params = Self;
	type Result = ListRootsResultData;
}

// endregion: --- List Roots Request

// region:    --- List Roots Result

/// Specific result data for the ListRoots response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListRootsResultData {
	pub roots: Vec<Root>,
}

/// Type alias for the full ListRoots Result structure using the wrapper.
pub type ListRootsResult = ResultData<ListRootsResultData>;

/// Represents a root directory or file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
	/// The URI identifying the root (must start with file:// for now).
	pub uri: String,

	/// Optional human-readable name for the root.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

// endregion: --- List Roots Result

// region:    --- Roots List Changed Notification

/// Parameters for the RootsListChangedNotification (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RootsListChangedParams {}

/// Type alias for the full RootsListChanged Notification structure using the wrapper.
pub type RootsListChangedNotification = NotificationParams<RootsListChangedParams>;

/// Associates the params structure with the Notification trait.
impl Notification for RootsListChangedParams {
	const METHOD: &'static str = "notifications/roots/list_changed";
	type Params = Self;
}

// endregion: --- Roots List Changed Notification
