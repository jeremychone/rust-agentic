use crate::mcp::Annotations; // Updated import
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none};

/// A known resource that the server is capable of reading.
///
/// TS Ref: `Resource`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
	/// The URI of this resource.
	/// @format uri
	pub uri: String,

	/// A human-readable name for this resource.
	/// This can be used by clients to populate UI elements.
	pub name: String,

	/// A description of what this resource represents.
	/// This can be used by clients to improve the LLM's understanding of available resources.
	pub description: Option<String>,

	/// The MIME type of this resource, if known.
	pub mime_type: Option<String>,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,

	/// The size of the raw resource content, in bytes (i.e., before base64 encoding or any tokenization), if known.
	/// This can be used by Hosts to display file sizes and estimate context window usage.
	pub size: Option<i64>,
}

/// Builders
impl Resource {
	pub fn new(uri: impl Into<String>, name: impl Into<String>) -> Self {
		Self {
			uri: uri.into(),
			name: name.into(),
			description: None,
			mime_type: None,
			annotations: None,
			size: None,
		}
	}

	pub fn with_description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}

	pub fn with_annotations(mut self, annotations: Annotations) -> Self {
		self.annotations = Some(annotations);
		self
	}

	pub fn with_size(mut self, size: i64) -> Self {
		self.size = Some(size);
		self
	}
}

/// A template description for resources available on the server.
///
/// TS Ref: `ResourceTemplate`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTemplate {
	/// A URI template (according to RFC 6570) that can be used to construct resource URIs.
	/// @format uri-template
	pub uri_template: String,

	/// A human-readable name for the type of resource this template refers to.
	/// This can be used by clients to populate UI elements.
	pub name: String,

	/// A description of what this template is for.
	/// This can be used by clients to improve the LLM's understanding of available resources.
	pub description: Option<String>,

	/// The MIME type for all resources that match this template.
	/// This should only be included if all resources matching this template have the same type.
	pub mime_type: Option<String>,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// Builders
impl ResourceTemplate {
	pub fn new(uri_template: impl Into<String>, name: impl Into<String>) -> Self {
		Self {
			uri_template: uri_template.into(),
			name: name.into(),
			description: None,
			mime_type: None,
			annotations: None,
		}
	}

	pub fn with_description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn with_mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}

	pub fn with_annotations(mut self, annotations: Annotations) -> Self {
		self.annotations = Some(annotations);
		self
	}
}

/// The contents of a specific resource or sub-resource.
///
/// JC-NOTE: Not sure why the TS Spec had a 's' on this one. Only one content
///
/// TS Ref: `ResourceContents`
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ResourceContents {
	#[serde(rename_all = "camelCase")]
	Text {
		/// The URI of this resource.
		/// @format uri
		uri: String,

		/// The MIME type of this resource, if known.
		#[serde(skip_serializing_if = "Option::is_none")]
		mime_type: Option<String>,

		/// The text of the item.
		text: String,
	},

	#[serde(rename_all = "camelCase")]
	Blob {
		/// The URI of this resource.
		/// @format uri
		uri: String,

		/// The MIME type of this resource, if known.
		#[serde(skip_serializing_if = "Option::is_none")]
		mime_type: Option<String>,

		/// A base64-encoded string representing the binary data of the item.
		/// @format byte
		#[serde_as(as = "Base64")]
		blob: Vec<u8>,
	},
}

/// Represents a root directory or file that the server can operate on.
///
/// TS Ref: `Root`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
	/// The URI identifying the root. This *must* start with file:// for now.
	/// This restriction may be relaxed in future versions of the protocol to allow
	/// other URI schemes.
	/// @format uri
	pub uri: String,

	/// An optional name for the root. This can be used to provide a human-readable
	/// identifier for the root, which may be useful for display purposes or for
	/// referencing the root in other parts of the application.
	pub name: Option<String>,
}

/// Builders
impl Root {
	pub fn new(uri: impl Into<String>) -> Self {
		Self {
			uri: uri.into(),
			name: None,
		}
	}

	pub fn with_name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
