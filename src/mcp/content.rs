//! Types defining content blocks used in messages (Text, Image, Audio, EmbeddedResource).

use super::base::Annotations;
use serde::{Deserialize, Serialize};
use serde_with::serde_as; // Required for base64

// region:    --- Basic Content Types

/// Text provided to or from an LLM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TextContent {
	/// The text content of the message.
	pub text: String,
	/// Optional annotations for the client.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Annotations>,
}

/// An image provided to or from an LLM.
#[serde_as] // Apply serde_as to the struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
	/// The base64-encoded image data.
	#[serde_as(as = "serde_with::base64::Base64")] // Specify Base64 encoding for this field
	pub data: Vec<u8>,
	/// The MIME type of the image.
	pub mime_type: String,
	/// Optional annotations for the client.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Annotations>,
}

/// Audio provided to or from an LLM.
#[serde_as] // Apply serde_as to the struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioContent {
	/// The base64-encoded audio data.
	#[serde_as(as = "serde_with::base64::Base64")] // Specify Base64 encoding for this field
	pub data: Vec<u8>,
	/// The MIME type of the audio.
	pub mime_type: String,
	/// Optional annotations for the client.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Annotations>,
}

// endregion: --- Basic Content Types

// region:    --- Resource Content Types

/// The contents of a resource, embedded into a prompt or tool call result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedResource {
	// Removed `kind: String` field, rely on enum tag
	/// The actual resource contents.
	pub resource: ResourceContentsVariant,
	/// Optional annotations for the client.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Annotations>,
}

// Default implementation
impl Default for EmbeddedResource {
	fn default() -> Self {
		Self {
			resource: ResourceContentsVariant::Text(Default::default()),
			annotations: None,
		}
	}
}

/// Base properties for resource contents.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceContentsBase {
	/// The URI of this resource.
	pub uri: String,
	/// The MIME type of this resource, if known.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mime_type: Option<String>,
}

/// Text-based resource contents.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TextResourceContents {
	#[serde(flatten)]
	pub base: ResourceContentsBase,
	/// The text of the item.
	pub text: String,
}

/// Binary blob resource contents.
#[serde_as] // Apply serde_as to the struct
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BlobResourceContents {
	#[serde(flatten)]
	pub base: ResourceContentsBase,
	/// A base64-encoded string representing the binary data.
	#[serde_as(as = "serde_with::base64::Base64")] // Specify Base64 encoding for this field
	pub blob: Vec<u8>,
}

/// Enum to represent either Text or Blob resource contents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)] // Determine variant based on presence of 'text' or 'blob' keys
pub enum ResourceContentsVariant {
	Text(TextResourceContents),
	Blob(BlobResourceContents),
}

// endregion: --- Resource Content Types
