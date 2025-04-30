use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;

// Helper function to serialize a boolean field as an empty object {} if true,
// and skip it entirely if false.
pub fn serialize_bool_as_empty_object<S>(
	map: &mut S::SerializeMap,
	key: &'static str,
	value: bool,
) -> Result<(), S::Error>
where
	S: Serializer,
	S::SerializeMap: SerializeMap,
{
	if value {
		map.serialize_entry(key, &serde_json::json!({}))?;
	}
	// If false, do nothing (skip serialization)
	Ok(())
}

// Helper function to check if a Value is an empty JSON object `{}`
pub fn is_empty_object(value: &Value) -> bool {
	value.is_object() && value.as_object().is_some_and(|m| m.is_empty())
}

// region:    --- Text

pub fn truncate(s: &str, max: usize) -> String {
	if s.chars().count() <= max {
		s.to_string()
	} else {
		let mut result = String::new();
		for (i, c) in s.chars().enumerate() {
			if i >= max {
				result.push_str("...");
				break;
			}
			result.push(c);
		}
		result
	}
}

// endregion: --- Text
