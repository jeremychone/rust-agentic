// region:    --- Modules

mod server_io_trx;
mod transport;

use server_io_trx::*;

mod client_impl;
mod error;

pub use client_impl::*;
pub use error::{Error, Result};

pub use transport::ClientStdioTransportConfig;

// endregion: --- Modules
