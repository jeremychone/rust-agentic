//! Utility-related notifications like logging and progress updates.

use crate::mcp::{GenericMeta, LoggingLevel, Notification, ProgressToken};
use serde::{Deserialize, Serialize};
use serde_json::Value;
