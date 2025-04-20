//! Types related to the ping request.

use super::{EmptyResultData, Request, RequestParams}; // Use re-exports
use serde::{Deserialize, Serialize};

/// Parameters for the PingRequest (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PingParams {}

/// Type alias for the full Ping Request structure using the wrapper.
pub type PingRequest = RequestParams<PingParams>;

/// Associates the params/result structures with the Request trait.
impl Request for PingParams {
	const METHOD: &'static str = "ping";

	type Params = Self;

	type Result = EmptyResultData;
}

// Note: Ping response uses EmptyResultData, handled within ServerResultSpecificData/ClientResultSpecificData enums.
// No separate PingResult type alias is needed here.
