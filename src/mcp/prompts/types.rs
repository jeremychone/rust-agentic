use crate::mcp::{Annotations, ResourceContents, Role}; // Updated ResourceContents import
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::{serde_as, skip_serializing_none};

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

/// Content types for messages (used in Prompts and Tool Calls).
///
/// TS Ref: various content types (TextContent, ImageContent, AudioContent, EmbeddedResource)
#[serde_as]
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
		#[serde_as(as = "Base64")]
		data: Vec<u8>,

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
		#[serde_as(as = "Base64")]
		data: Vec<u8>,

		/// The MIME type of the audio. Different providers may support different audio types.
		mime_type: String,

		/// Optional annotations for the client.
		#[serde(skip_serializing_if = "Option::is_none")]
		annotations: Option<Annotations>,
	},

	/// The contents of a resource, embedded into a prompt or tool call result.
	///
	/// TS Ref: `EmbeddedResource`
	#[serde(rename_all = "camelCase")]
	Resource {
		/// The resource content
		resource: ResourceContents, // Uses ResourceContents enum

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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Prompt {
	/// The name of the prompt or prompt template.
	pub name: String,

	/// An optional description of what this prompt provides
	pub description: Option<String>,

	/// A list of arguments to use for templating the prompt.
	pub arguments: Option<Vec<PromptArgument>>,
}

/// Identifies a prompt for completion context.
///
/// TS Ref: `PromptReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptReference {
	/// The name of the prompt or prompt template
	pub name: String,
}
