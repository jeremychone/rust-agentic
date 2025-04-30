use rpc_router::{RpcId, RpcRequest, RpcRequestCheckFlags};
use serde::Deserialize;
use serde::de::{Deserializer, Error as DeError};
use serde::ser::{Error as SerError, SerializeMap, Serializer};
use serde_json::Value;

const JSONRPC_VERSION: &str = "2.0";

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

/// Helper to build the base JSON-RPC request map for serialization.
pub fn base_rpc_request_map<S: Serializer>(
	serializer: S,
	method: &str,
	rpc_id: Option<RpcId>, // Use Option for Notifications (no id)
	params: Option<Value>,
) -> Result<S::SerializeMap, S::Error> {
	let mut num_fields = 2; // jsonrpc, method
	if rpc_id.is_some() {
		num_fields += 1; // id
	}
	if params.is_some() {
		num_fields += 1; // params
	}

	let mut map = serializer.serialize_map(Some(num_fields))?;

	map.serialize_entry("jsonrpc", JSONRPC_VERSION)?;
	map.serialize_entry("method", method)?;

	if let Some(rpc_id) = rpc_id {
		map.serialize_entry("id", &rpc_id)?;
	}

	if let Some(params) = params {
		map.serialize_entry("params", &params)?;
	}
	Ok(map)
}

/// Helper for deserializing the base structure of JSON-RPC Requests.
/// Validates version, presence of ID, and method.
pub(crate) fn deserialize_request_base<'de, D>(
	deserializer: D,
	expected_method: &'static str,
) -> Result<RpcRequest, D::Error>
where
	D: Deserializer<'de>,
{
	// Step 1: Deserialize into a generic Value
	let value = Value::deserialize(deserializer)?;

	// Step 2: Use RpcRequest::from_value_with_checks to parse the structure
	// Check for version and ID (requests MUST have an ID)
	let request = RpcRequest::from_value_with_checks(value, RpcRequestCheckFlags::VERSION | RpcRequestCheckFlags::ID)
		.map_err(|e| DeError::custom(format!("Failed to parse base JSON-RPC request structure: {}", e)))?;

	// Step 3: Validate Request specific constraints
	// - ID must NOT be Null (checked by RpcRequestCheckFlags::ID implicitly)
	// - Must have the correct method
	if request.method != expected_method {
		return Err(DeError::custom(format!(
			"Invalid method: {}, expected: {}",
			request.method, expected_method
		)));
	}

	Ok(request)
}

/// Helper to validate that the `params` field is absent, null, or an empty object.
pub(crate) fn validate_params_absence<E: DeError>(params_value: &Option<Value>) -> Result<(), E> {
	if let Some(params) = params_value {
		if !(params.is_null() || (params.is_object() && params.as_object().unwrap().is_empty())) {
			return Err(E::custom("Expected empty object, null, or absent 'params' field"));
		}
	}
	// If params is None, it's valid.
	Ok(())
}

/// Helper for deserializing the base structure of JSON-RPC Notifications.
/// Validates version, absence of ID, and method.
pub(crate) fn deserialize_notification_base<'de, D>(
	deserializer: D,
	expected_method: &'static str,
) -> Result<RpcRequest, D::Error>
where
	D: Deserializer<'de>,
{
	// Step 1: Deserialize into a generic Value
	let value = Value::deserialize(deserializer)?;

	// Step 2: Use RpcRequest::from_value_with_checks to parse the structure
	// We only check for the JSON-RPC version, skipping the ID check initially.
	let request = RpcRequest::from_value_with_checks(value, RpcRequestCheckFlags::VERSION)
		.map_err(|e| DeError::custom(format!("Failed to parse base JSON-RPC structure: {}", e)))?;

	// Step 3: Validate Notification specific constraints
	// - Must NOT have an ID (or it must be Null from parsing perspective if ID check skipped)
	if request.id != RpcId::Null {
		// Note: from_value_with_checks defaults to Null if missing and check skipped.
		// If it's *not* Null, it means an ID was actually present in the JSON.
		return Err(DeError::custom("Invalid 'id' field found in Notification"));
	}
	// - Must have the correct method
	if request.method != expected_method {
		return Err(DeError::custom(format!(
			"Invalid method: {}, expected: {}",
			request.method, expected_method
		)));
	}
	Ok(request)
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
