use crate::mcp::{Annotations, Cursor, GenericMeta, PaginationParams, RequestMeta, ResourceContents, Role};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Describes a message returned as part of a prompt.
///
/// This is similar to `SamplingMessage`, but also supports the embedding of
/// resources from the MCP server.
///
/// TS Ref: `PromptMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptMessage {
	pub role: Role,
	pub content: MessageContent,
}

/// Content types for messages
///
/// TS Ref: various content types (TextContent, ImageContent, AudioContent, EmbeddedResource)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MessageContent {
	#[serde(rename_all = "camelCase")]
	Text {
		/// The text content of the message.
		text: String,

		/// Optional annotations for the client.
		#[serde(skip_serializing_if = "Option::is_none")]
		annotations: Option<Annotations>,
	},

	#[serde(rename_all = "camelCase")]
	Image {
		/// The base64-encoded image data.
		/// @format byte
		data: String,

		/// The MIME type of the image. Different providers may support different image types.
		mime_type: String,

		/// Optional annotations for the client.
		#[serde(skip_serializing_if = "Option::is_none")]
		annotations: Option<Annotations>,
	},

	#[serde(rename_all = "camelCase")]
	Audio {
		/// The base64-encoded audio data.
		/// @format byte
		data: String,

		/// The MIME type of the audio. Different providers may support different audio types.
		mime_type: String,

		/// Optional annotations for the client.
		#[serde(skip_serializing_if = "Option::is_none")]
		annotations: Option<Annotations>,
	},

	#[serde(rename_all = "camelCase")]
	Resource {
		/// The resource content
		resource: ResourceContents,

		/// Optional annotations for the client.
		#[serde(skip_serializing_if = "Option::is_none")]
		annotations: Option<Annotations>,
	},
}

/// Describes an argument that a prompt can accept.
///
/// TS Ref: `PromptArgument`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
	/// The name of the argument.
	pub name: String,

	/// A human-readable description of the argument.
	pub description: Option<String>,

	/// Whether this argument must be provided.
	pub required: Option<bool>,
}

/// A prompt or prompt template that the server offers.
///
/// TS Ref: `Prompt`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
	/// The name of the prompt or prompt template.
	pub name: String,

	/// An optional description of what this prompt provides
	pub description: Option<String>,

	/// A list of arguments to use for templating the prompt.
	pub arguments: Option<Vec<PromptArgument>>,
}
