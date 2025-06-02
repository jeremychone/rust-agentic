use derive_more::{Display, From};
use flume::{RecvError, SendError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),

	CommSend(String),
	CommRecv(RecvError),

	// -- Externals
	#[from]
	Reqwest(reqwest::Error),
}

// region:    --- Froms

impl From<SendError<String>> for Error {
	fn from(value: SendError<String>) -> Self {
		Self::CommSend(value.to_string())
	}
}

impl From<RecvError> for Error {
	fn from(err: RecvError) -> Self {
		Self::CommRecv(err)
	}
}

// endregion: --- Froms

// region:    --- Intos

impl From<Error> for crate::mcp::Error {
	fn from(value: Error) -> Self {
		crate::mcp::Error::Transport(value.to_string())
	}
}

// endregion: --- Intos

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
