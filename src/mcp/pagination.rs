//! Types related to pagination (cursors, params, results).

use super::base::Cursor; // Use Cursor from base
use serde::{Deserialize, Serialize};

/// Parameters for requests that support pagination (used as the `specific` part in `RequestParams`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PaginatedParams {
	/// Opaque token for the current pagination position.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cursor: Option<Cursor>,
}

/// Result structure for responses that support pagination (used within the specific result data).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResultData {
	/// Opaque token for the next page, if available.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next_cursor: Option<Cursor>,
}
