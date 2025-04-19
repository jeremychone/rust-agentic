//! Model Context Protocol (MCP) types based on the TypeScript schema.

// region:    --- Modules

// -- Core & Common Types
mod base;
mod content;
mod pagination;
mod traits;
mod wrappers;

// -- Message Types (Grouped)
mod autocomplete;
mod cancel;
mod init;
mod logging;
mod ping;
mod progress;
mod prompts;
mod resources;
mod roots;
mod sampling;
mod tools;

// -- Message Aggregators
mod client_messages;
mod server_messages;

// Flatten the exports
pub use base::*;
pub use content::*;
pub use pagination::*;
pub use traits::*;
pub use wrappers::*;

pub use autocomplete::*;
pub use cancel::*;
pub use init::*;
pub use logging::*;
pub use ping::*;
pub use progress::*;
pub use prompts::*;
pub use resources::*;
pub use roots::*;
pub use sampling::*;
pub use tools::*;

pub use client_messages::*;
pub use server_messages::*;

// endregion: --- Modules
