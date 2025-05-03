use crate::mcp::{Annotations, Role};
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;

/// Hints to use for model selection.
///
/// Keys not declared here are currently left unspecified by the spec and are up
/// to the client to interpret.
///
/// TS Ref: `ModelHint`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelHint {
	/// A hint for a model name.
	pub name: Option<String>,
	// Note: Does not capture arbitrary extra fields as per schema description.
}

/// Builders
impl ModelHint {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}

/// The server's preferences for model selection, requested of the client during sampling.
///
/// These preferences are always advisory. The client MAY ignore them.
///
/// TS Ref: `ModelPreferences`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelPreferences {
	/// Optional hints to use for model selection.
	pub hints: Option<Vec<ModelHint>>,

	/// How much to prioritize cost when selecting a model (0-1).
	pub cost_priority: Option<f64>,

	/// How much to prioritize sampling speed (latency) when selecting a model (0-1).
	pub speed_priority: Option<f64>,

	/// How much to prioritize intelligence and capabilities when selecting a model (0-1).
	pub intelligence_priority: Option<f64>,
}

/// Builders
impl ModelPreferences {
	/// Same as default (for API consistency)
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_hints(mut self, hints: Vec<ModelHint>) -> Self {
		self.hints = Some(hints);
		self
	}

	pub fn append_hint(mut self, hint: ModelHint) -> Self {
		self.hints.get_or_insert_with(Vec::new).push(hint);
		self
	}

	pub fn with_cost_priority(mut self, priority: f64) -> Self {
		self.cost_priority = Some(priority);
		self
	}

	pub fn with_speed_priority(mut self, priority: f64) -> Self {
		self.speed_priority = Some(priority);
		self
	}

	pub fn with_intelligence_priority(mut self, priority: f64) -> Self {
		self.intelligence_priority = Some(priority);
		self
	}
}

/// Text provided to or from an LLM.
///
/// TS Ref: `TextContent`
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
	/// The text content of the message.
	pub text: String,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// Builders
impl TextContent {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			annotations: None,
		}
	}

	pub fn with_annotations(mut self, annotations: Annotations) -> Self {
		self.annotations = Some(annotations);
		self
	}
}

/// An image provided to or from an LLM.
///
/// TS Ref: `ImageContent`
#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageContent {
	/// The base64-encoded image data.
	/// @format byte
	#[serde_as(as = "Base64")]
	pub data: Vec<u8>,

	/// The MIME type of the image.
	pub mime_type: String,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// Builders
impl ImageContent {
	pub fn new(data: Vec<u8>, mime_type: impl Into<String>) -> Self {
		Self {
			data,
			mime_type: mime_type.into(),
			annotations: None,
		}
	}

	pub fn with_annotations(mut self, annotations: Annotations) -> Self {
		self.annotations = Some(annotations);
		self
	}
}

/// Audio provided to or from an LLM.
///
/// TS Ref: `AudioContent`
#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioContent {
	/// The base64-encoded audio data.
	/// @format byte
	#[serde_as(as = "Base64")]
	pub data: Vec<u8>,

	/// The MIME type of the audio.
	pub mime_type: String,

	/// Optional annotations for the client.
	pub annotations: Option<Annotations>,
}

/// Builders
impl AudioContent {
	pub fn new(data: Vec<u8>, mime_type: impl Into<String>) -> Self {
		Self {
			data,
			mime_type: mime_type.into(),
			annotations: None,
		}
	}

	pub fn with_annotations(mut self, annotations: Annotations) -> Self {
		self.annotations = Some(annotations);
		self
	}
}

/// Represents the content part of a `SamplingMessage`.
/// Based on TS `SamplingMessage.content: TextContent | ImageContent | AudioContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SamplingContent {
	Text(TextContent),
	Image(ImageContent),
	Audio(AudioContent),
}

/// Builders / Constructors (for convenience, often direct construction is fine)
impl SamplingContent {
	pub fn new_text(text: impl Into<String>) -> Self {
		Self::Text(TextContent::new(text))
	}

	pub fn new_image(data: Vec<u8>, mime_type: impl Into<String>) -> Self {
		Self::Image(ImageContent::new(data, mime_type))
	}

	pub fn new_audio(data: Vec<u8>, mime_type: impl Into<String>) -> Self {
		Self::Audio(AudioContent::new(data, mime_type))
	}

	/// Adds annotations to the inner content struct if applicable.
	pub fn with_annotations(mut self, annotations: Annotations) -> Self {
		match &mut self {
			Self::Text(c) => c.annotations = Some(annotations),
			Self::Image(c) => c.annotations = Some(annotations),
			Self::Audio(c) => c.annotations = Some(annotations),
		}
		self
	}
}

/// Describes a message issued to or received from an LLM API.
///
/// TS Ref: `SamplingMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingMessage {
	pub role: Role,
	pub content: SamplingContent,
}

/// Builders
impl SamplingMessage {
	pub fn new(role: Role, content: SamplingContent) -> Self {
		Self { role, content }
	}

	// No specific 'with_' methods needed as fields are public and required at construction.
}

