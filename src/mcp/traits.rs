//! Core MCP traits for Requests and Notifications.

/// Base trait for MCP Requests, defining the method name and associated param/result types.
/// Note: `Params` and `Result` here refer to the *specific* data structures,
/// not the `RequestParams<T>` or `ResultData<T>` wrappers.
pub trait Request {
	const METHOD: &'static str;
	type Params; // The specific parameters struct (e.g., InitializeParams)
	type Result; // The specific result struct (e.g., InitializeResultData)
}

/// Base trait for MCP Notifications, defining the method name and associated param type.
/// Note: `Params` here refers to the *specific* data structure,
/// not the `NotificationParams<T>` wrapper.
pub trait Notification {
	const METHOD: &'static str;
	type Params; // The specific parameters struct (e.g., CancelledParams)
}
