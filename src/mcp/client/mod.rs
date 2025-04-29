// region:    --- Modules

mod coms_trx;
mod transport;

use coms_trx::*;

mod client_impl;
mod error;

pub use client_impl::*;
pub use error::{Error, Result};

pub use transport::ClientStdioTransportConfig;

// endregion: --- Modules
