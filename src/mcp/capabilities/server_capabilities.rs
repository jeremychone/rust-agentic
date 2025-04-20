use crate::mcp::support::{is_empty_object, serialize_bool_as_empty_object};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_with::skip_serializing_none; // <-- Ensure this is imported
use std::fmt;

/// Capabilities that a server may support. Known capabilities are defined here, in this schema,
/// but this is not a closed set: any server can define its own, additional capabilities.
///
/// TS Ref: `ServerCapabilities`
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServerCapabilities {
	/// Experimental, non-standard capabilities that the server supports.
	pub experimental: Option<Value>,

	/// Present if the server supports sending log messages to the client.
	/// Represented as an empty JSON object `{}` when present in JSON.
	pub logging: bool,

	/// Present if the server supports argument autocompletion suggestions.
	/// Represented as an empty JSON object `{}` when present in JSON.
	pub completions: bool,

	/// Present if the server offers any prompt templates.
	pub prompts: Option<ServerPromptsCapabilities>,

	/// Present if the server offers any resources to read.
	pub resources: Option<ServerResourcesCapabilities>,

	/// Present if the server offers any tools to call.
	pub tools: Option<ServerToolsCapabilities>,
}

/// Capabilities related to prompts supported by the server.
/// Nested within `ServerCapabilities`.
///
/// TS Ref: `ServerCapabilities.prompts`
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerPromptsCapabilities {
	/// Whether this server supports notifications for changes to the prompt list.
	pub list_changed: Option<bool>,
}

/// Capabilities related to resources supported by the server.
/// Nested within `ServerCapabilities`.
///
/// TS Ref: `ServerCapabilities.resources`
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerResourcesCapabilities {
	/// Whether this server supports subscribing to resource updates.
	pub subscribe: Option<bool>,

	/// Whether this server supports notifications for changes to the resource list.
	pub list_changed: Option<bool>,
}

/// Capabilities related to tools supported by the server.
/// Nested within `ServerCapabilities`.
///
/// TS Ref: `ServerCapabilities.tools`
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerToolsCapabilities {
	/// Whether this server supports notifications for changes to the tool list.
	pub list_changed: Option<bool>,
}

// -- Manual Serialize for ServerCapabilities
impl Serialize for ServerCapabilities {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(None)?;

		if let Some(experimental) = &self.experimental {
			map.serialize_entry("experimental", experimental)?;
		}
		// Use helper for logging: serialize as {} if true, skip if false
		serialize_bool_as_empty_object::<S>(&mut map, "logging", self.logging)?;
		// Use helper for completions: serialize as {} if true, skip if false
		serialize_bool_as_empty_object::<S>(&mut map, "completions", self.completions)?;

		if let Some(prompts) = &self.prompts {
			map.serialize_entry("prompts", prompts)?;
		}
		if let Some(resources) = &self.resources {
			map.serialize_entry("resources", resources)?;
		}
		if let Some(tools) = &self.tools {
			map.serialize_entry("tools", tools)?;
		}

		map.end()
	}
}

// -- Manual Deserialize for ServerCapabilities
impl<'de> Deserialize<'de> for ServerCapabilities {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct ServerCapabilitiesVisitor;

		impl<'de> Visitor<'de> for ServerCapabilitiesVisitor {
			type Value = ServerCapabilities;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a map representing ServerCapabilities")
			}

			fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
			where
				M: MapAccess<'de>,
			{
				let mut experimental: Option<Value> = None;
				let mut logging: bool = false; // Default to false
				let mut completions: bool = false; // Default to false
				let mut prompts: Option<ServerPromptsCapabilities> = None;
				let mut resources: Option<ServerResourcesCapabilities> = None;
				let mut tools: Option<ServerToolsCapabilities> = None;

				while let Some(key) = map.next_key::<String>()? {
					match key.as_str() {
						"experimental" => {
							experimental = Some(map.next_value()?);
						}
						"logging" => {
							let logging_value: Value = map.next_value()?;
							if is_empty_object(&logging_value) {
								logging = true;
							}
						}
						"completions" => {
							let completions_value: Value = map.next_value()?;
							if is_empty_object(&completions_value) {
								completions = true;
							}
						}
						"prompts" => {
							prompts = Some(map.next_value()?);
						}
						"resources" => {
							resources = Some(map.next_value()?);
						}
						"tools" => {
							tools = Some(map.next_value()?);
						}
						_ => {
							let _ = map.next_value::<Value>()?;
						}
					}
				}

				Ok(ServerCapabilities {
					experimental,
					logging,
					completions,
					prompts,
					resources,
					tools,
				})
			}
		}

		deserializer.deserialize_map(ServerCapabilitiesVisitor)
	}
}
