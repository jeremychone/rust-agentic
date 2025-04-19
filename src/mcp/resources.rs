//! Types related to resource listing, reading, and subscriptions.

use super::{
	Annotations, EmptyResultData, Notification, NotificationParams, PaginatedParams, PaginatedResultData, Request,
	RequestParams, ResourceContentsVariant, ResultData,
}; // Use re-exports
use serde::{Deserialize, Serialize};

// region:    --- List Resources

/// Specific parameters for listing resources, including pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListResourcesParams {
	#[serde(flatten)]
	pub pagination: PaginatedParams,
}

/// Type alias for the full ListResources Request structure using the wrapper.
pub type ListResourcesRequest = RequestParams<ListResourcesParams>;

/// Specific result data for listing resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourcesResultData {
	#[serde(flatten)]
	pub pagination: PaginatedResultData,
	pub resources: Vec<Resource>,
}

/// Type alias for the full ListResources Result structure using the wrapper.
pub type ListResourcesResult = ResultData<ListResourcesResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for ListResourcesParams {
	const METHOD: &'static str = "resources/list";
	type Params = Self;
	type Result = ListResourcesResultData;
}

// endregion: --- List Resources

// region:    --- List Resource Templates

/// Specific parameters for listing resource templates, including pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ListResourceTemplatesParams {
	#[serde(flatten)]
	pub pagination: PaginatedParams,
}

/// Type alias for the full ListResourceTemplates Request structure using the wrapper.
pub type ListResourceTemplatesRequest = RequestParams<ListResourceTemplatesParams>;

/// Specific result data for listing resource templates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResourceTemplatesResultData {
	#[serde(flatten)]
	pub pagination: PaginatedResultData,
	pub resource_templates: Vec<ResourceTemplate>,
}

/// Type alias for the full ListResourceTemplates Result structure using the wrapper.
pub type ListResourceTemplatesResult = ResultData<ListResourceTemplatesResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for ListResourceTemplatesParams {
	const METHOD: &'static str = "resources/templates/list";
	type Params = Self;
	type Result = ListResourceTemplatesResultData;
}

// endregion: --- List Resource Templates

// region:    --- Read Resource

/// Specific parameters for reading a resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadResourceParams {
	/// The URI of the resource to read.
	pub uri: String,
}

/// Type alias for the full ReadResource Request structure using the wrapper.
pub type ReadResourceRequest = RequestParams<ReadResourceParams>;

/// Specific result data for reading a resource.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadResourceResultData {
	/// A list of resource contents (can be multiple if the URI resolved to multiple parts).
	pub contents: Vec<ResourceContentsVariant>,
}

/// Type alias for the full ReadResource Result structure using the wrapper.
pub type ReadResourceResult = ResultData<ReadResourceResultData>;

/// Associates the params/result structures with the Request trait.
impl Request for ReadResourceParams {
	const METHOD: &'static str = "resources/read";
	type Params = Self;
	type Result = ReadResourceResultData;
}

// endregion: --- Read Resource

// region:    --- Resource List Changed Notification

/// Parameters for the ResourceListChangedNotification (currently none).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceListChangedParams {}

/// Type alias for the full ResourceListChanged Notification structure using the wrapper.
pub type ResourceListChangedNotification = NotificationParams<ResourceListChangedParams>;

/// Associates the params structure with the Notification trait.
impl Notification for ResourceListChangedParams {
	const METHOD: &'static str = "notifications/resources/list_changed";
	type Params = Self;
}

// endregion: --- Resource List Changed Notification

// region:    --- Subscribe Request

/// Specific parameters for subscribing to resource updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeParams {
	/// The URI of the resource to subscribe to.
	pub uri: String,
}

/// Type alias for the full Subscribe Request structure using the wrapper.
pub type SubscribeRequest = RequestParams<SubscribeParams>;

/// Associates the params/result structures with the Request trait.
impl Request for SubscribeParams {
	const METHOD: &'static str = "resources/subscribe";
	type Params = Self;
	type Result = EmptyResultData; // Successful subscription returns no data
}

// endregion: --- Subscribe Request

// region:    --- Unsubscribe Request

/// Specific parameters for unsubscribing from resource updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeParams {
	/// The URI of the resource to unsubscribe from.
	pub uri: String,
}

/// Type alias for the full Unsubscribe Request structure using the wrapper.
pub type UnsubscribeRequest = RequestParams<UnsubscribeParams>;

/// Associates the params/result structures with the Request trait.
impl Request for UnsubscribeParams {
	const METHOD: &'static str = "resources/unsubscribe";
	type Params = Self;
	type Result = EmptyResultData; // Successful unsubscription returns no data
}

// endregion: --- Unsubscribe Request

// region:    --- Resource Updated Notification

/// Specific parameters for the ResourceUpdatedNotification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceUpdatedParams {
	/// The URI of the resource that has been updated.
	pub uri: String,
}

/// Type alias for the full ResourceUpdated Notification structure using the wrapper.
pub type ResourceUpdatedNotification = NotificationParams<ResourceUpdatedParams>;

/// Associates the params structure with the Notification trait.
impl Notification for ResourceUpdatedParams {
	const METHOD: &'static str = "notifications/resources/updated";
	type Params = Self;
}

// endregion: --- Resource Updated Notification

// region:    --- Resource Definitions

/// A known resource the server can read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
	/// The URI of this resource.
	pub uri: String,
	/// A human-readable name for this resource.
	pub name: String,
	/// A description of what this resource represents.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The MIME type of this resource, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mime_type: Option<String>,
	/// Optional annotations for the client.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Annotations>,
	/// The size of the raw resource content in bytes, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<u64>, // Using u64 for size
}

/// A template description for resources available on the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTemplate {
	/// A URI template (RFC 6570) for resource URIs.
	pub uri_template: String,
	/// A human-readable name for the type of resource.
	pub name: String,
	/// A description of what this template is for.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// The MIME type for resources matching this template, if consistent.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mime_type: Option<String>,
	/// Optional annotations for the client.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Annotations>,
}

// endregion: --- Resource Definitions
