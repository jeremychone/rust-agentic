use super::types::{Resource, ResourceContents, ResourceTemplate};
use crate::mcp::{Cursor, GenericMeta, IntoMcpRequest, PaginationParams, RequestMeta};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// region:    --- ListResourcesRequest

/// Sent from the client to request a list of resources the server has.
///
/// TS Ref: `ListResourcesRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// Cursor for pagination
	#[serde(flatten)]
	pub pagination: PaginationParams,
}

/// Builders
impl ListResourcesParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
		self.pagination = pagination;
		self
	}

	pub fn with_cursor(mut self, cursor: impl Into<Cursor>) -> Self {
		self.pagination.cursor = Some(cursor.into());
		self
	}
}

impl IntoMcpRequest<ListResourcesParams> for ListResourcesParams {
	const METHOD: &'static str = "resources/list";
	type McpResult = ListResourcesResult;
}

/// The server's response to a resources/list request from the client.
///
/// TS Ref: `ListResourcesResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// An opaque token representing the pagination position after the last returned result.
	/// If present, there may be more results available.
	pub next_cursor: Option<Cursor>,

	/// The list of resources
	pub resources: Vec<Resource>,
}

// endregion: --- ListResourcesRequest

// region:    --- ListResourceTemplatesRequest

/// Sent from the client to request a list of resource templates the server has.
///
/// TS Ref: `ListResourceTemplatesRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTemplatesParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// Cursor for pagination
	#[serde(flatten)]
	pub pagination: PaginationParams,
}

/// Builders
impl ListResourceTemplatesParams {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}

	pub fn with_pagination(mut self, pagination: PaginationParams) -> Self {
		self.pagination = pagination;
		self
	}

	pub fn with_cursor(mut self, cursor: impl Into<Cursor>) -> Self {
		self.pagination.cursor = Some(cursor.into());
		self
	}
}

impl IntoMcpRequest<ListResourceTemplatesParams> for ListResourceTemplatesParams {
	const METHOD: &'static str = "resources/templates/list";
	type McpResult = ListResourceTemplatesResult;
}

/// The server's response to a resources/templates/list request from the client.
///
/// TS Ref: `ListResourceTemplatesResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTemplatesResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// An opaque token representing the pagination position after the last returned result.
	/// If present, there may be more results available.
	pub next_cursor: Option<Cursor>,

	/// The list of resource templates
	pub resource_templates: Vec<ResourceTemplate>,
}

// endregion: --- ListResourceTemplatesRequest

// region:    --- ReadResourceRequest

/// Sent from the client to the server, to read a specific resource URI.
///
/// TS Ref: `ReadResourceRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResourceParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.
	/// @format uri
	pub uri: String,
}

/// Builders
impl ReadResourceParams {
	pub fn new(uri: impl Into<String>) -> Self {
		Self {
			meta: None,
			uri: uri.into(),
		}
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpRequest<ReadResourceParams> for ReadResourceParams {
	const METHOD: &'static str = "resources/read";
	type McpResult = ReadResourceResult;
}

/// The server's response to a resources/read request from the client.
///
/// TS Ref: `ReadResourceResult`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResourceResult {
	/// Optional metadata
	#[serde(rename = "_meta")]
	pub meta: Option<GenericMeta>,

	/// The contents of the resource
	pub contents: Vec<ResourceContents>,
}

// endregion: --- ReadResourceRequest

// region:    --- SubscribeRequest

/// Sent from the client to request resources/updated notifications from the server whenever a particular resource changes.
///
/// TS Ref: `SubscribeRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.
	/// @format uri
	pub uri: String,
}

/// Builders
impl SubscribeParams {
	pub fn new(uri: impl Into<String>) -> Self {
		Self {
			meta: None,
			uri: uri.into(),
		}
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpRequest<SubscribeParams> for SubscribeParams {
	const METHOD: &'static str = "resources/subscribe";
	type McpResult = ();
}

// endregion: --- SubscribeRequest

// region:    --- UnsubscribeRequest

/// Sent from the client to request cancellation of resources/updated notifications from the server.
/// This should follow a previous resources/subscribe request.
///
/// TS Ref: `UnsubscribeRequest`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeParams {
	#[serde(rename = "_meta")]
	pub meta: Option<RequestMeta>,

	/// The URI of the resource to unsubscribe from.
	/// @format uri
	pub uri: String,
}

/// Builders
impl UnsubscribeParams {
	pub fn new(uri: impl Into<String>) -> Self {
		Self {
			meta: None,
			uri: uri.into(),
		}
	}

	pub fn with_meta(mut self, meta: RequestMeta) -> Self {
		self.meta = Some(meta);
		self
	}
}

impl IntoMcpRequest<UnsubscribeParams> for UnsubscribeParams {
	const METHOD: &'static str = "resources/unsubscribe";
	type McpResult = ();
}

// endregion: --- UnsubscribeRequest
