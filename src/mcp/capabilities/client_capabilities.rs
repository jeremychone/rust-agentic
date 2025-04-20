use crate::mcp::support::{is_empty_object, serialize_bool_as_empty_object};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_with::skip_serializing_none; // <-- Ensure this is imported
use std::fmt;

/// Capabilities a client may support. Known capabilities are defined here, in this schema,
/// but this is not a closed set: any client can define its own, additional capabilities.
///
/// TS Ref: `ClientCapabilities`
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientCapabilities {
	/// Experimental, non-standard capabilities that the client supports.
	pub experimental: Option<Value>, // Value allows any JSON object

	/// Present if the client supports listing roots.
	pub roots: Option<ClientRootsCapabilities>,

	/// Present if the client supports sampling from an LLM.
	/// Represented as an empty JSON object `{}` when present in JSON.
	pub sampling: bool,
}

/// Capabilities related to listing roots supported by the client.
/// Nested within `ClientCapabilities`.
///
/// TS Ref: `ClientCapabilities.roots`
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRootsCapabilities {
	/// Whether the client supports notifications for changes to the roots list.
	pub list_changed: Option<bool>,
}

// -- Manual Serialize for ClientCapabilities
impl Serialize for ClientCapabilities {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(None)?; // Start map, size unknown initially

		if let Some(experimental) = &self.experimental {
			map.serialize_entry("experimental", experimental)?;
		}
		if let Some(roots) = &self.roots {
			map.serialize_entry("roots", roots)?;
		}
		// Use helper for sampling: serialize as {} if true, skip if false
		serialize_bool_as_empty_object::<S>(&mut map, "sampling", self.sampling)?;

		map.end()
	}
}

// -- Manual Deserialize for ClientCapabilities
impl<'de> Deserialize<'de> for ClientCapabilities {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct ClientCapabilitiesVisitor;

		impl<'de> Visitor<'de> for ClientCapabilitiesVisitor {
			type Value = ClientCapabilities;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a map representing ClientCapabilities")
			}

			fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
			where
				M: MapAccess<'de>,
			{
				let mut experimental: Option<Value> = None;
				let mut roots: Option<ClientRootsCapabilities> = None;
				let mut sampling: bool = false; // Default to false

				while let Some(key) = map.next_key::<String>()? {
					match key.as_str() {
						"experimental" => {
							experimental = Some(map.next_value()?);
						}
						"roots" => {
							roots = Some(map.next_value()?);
						}
						"sampling" => {
							// Deserialize the value for sampling first
							let sampling_value: Value = map.next_value()?;
							// If it's an empty object {}, set sampling to true
							if is_empty_object(&sampling_value) {
								sampling = true;
							}
							// Otherwise, it remains false (default)
						}
						// Ignore unknown fields gracefully
						_ => {
							let _ = map.next_value::<Value>()?;
						}
					}
				}

				Ok(ClientCapabilities {
					experimental,
					roots,
					sampling,
				})
			}
		}

		deserializer.deserialize_map(ClientCapabilitiesVisitor)
	}
}
