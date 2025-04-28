// region:    --- Modules

mod error;
mod support;

mod capabilities;
mod common;
mod lifecycle;
mod messages;
mod notifications;
mod prompts;
mod resources;
mod roots;
mod sampling;
mod tools;

pub use capabilities::*;
pub use common::*;
pub use error::*;
pub use lifecycle::*;
pub use messages::*;
pub use notifications::*;
pub use prompts::*;
pub use resources::*;
pub use roots::*;
pub use sampling::*;
pub use tools::*;

pub mod client;

// endregion: --- Modules

pub const LATEST_PROTOCOL_VERSION: &str = "2025-03-26";
