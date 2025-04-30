use crate::mcp::McpError;
use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),

	#[from]
	McpError(McpError),

	// -- McpMessage Errors
	McpMessageNotAnObject,
	McpMessageInvalidStructure(String),
	#[from]
	McpMessageDeserialization {
		type_name: &'static str,
		source: serde_json::Error,
	},
	McpTryIntoFail {
		actual_type: &'static str,
		target_type: &'static str,
	},

	// -- Sub Modules
	Transport(String),
}

// region:    --- Custom

impl Error {
	pub fn custom_from_err(err: impl std::error::Error) -> Self {
		Self::Custom(err.to_string())
	}

	pub fn custom(val: impl Into<String>) -> Self {
		Self::Custom(val.into())
	}
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
