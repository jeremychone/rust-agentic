use crate::RpcId;
use crate::mcp::{Error, McpError, McpNotification, McpRequest, McpResponse, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Represents any valid MCP message (Request, Notification, Response, or Error).
#[derive(Debug, Clone)]
pub enum McpMessage {
	Request(McpRequest<Value>),
	Notification(McpNotification<Value>),
	Response(McpResponse<Value>),
	Error(McpError),
}

impl McpMessage {
	pub fn rpc_id(&self) -> Option<&RpcId> {
		match self {
			McpMessage::Request(req) => Some(&req.id),
			McpMessage::Notification(notif) => None,
			McpMessage::Response(resp) => Some(&resp.id),
			McpMessage::Error(err) => Some(&err.id),
		}
	}

	pub fn stringify(&self) -> Result<String> {
		serde_json::to_string(&self).map_err(Error::custom_from_err)
	}
	pub fn stringify_pretty(&self) -> Result<String> {
		serde_json::to_string_pretty(&self).map_err(Error::custom_from_err)
	}
}

impl McpMessage {
	/// Parses a `serde_json::Value` into an `McpMessage`.
	///
	/// This function handles the logic of determining the message type
	/// based on the JSON structure and attempts deserialization into the
	/// appropriate `Mcp*` type. It returns a `crate::mcp::Error` on failure.
	pub fn from_value(value: Value) -> Result<McpMessage> {
		if let Some(obj) = value.as_object() {
			if obj.contains_key("result") {
				// Try deserialize as McpResponse
				McpResponse::deserialize(value).map(McpMessage::Response).map_err(|e| {
					Error::McpMessageDeserialization {
						type_name: "McpResponse",
						source: e,
					}
				})
			} else if obj.contains_key("error") {
				// Try deserialize as McpError
				McpError::deserialize(value)
					.map(McpMessage::Error)
					.map_err(|e| Error::McpMessageDeserialization {
						type_name: "McpError",
						source: e,
					})
			} else if obj.contains_key("method") {
				if obj.contains_key("id") && !obj.get("id").unwrap().is_null() {
					// Try deserialize as McpRequest (must have non-null id)
					McpRequest::deserialize(value).map(McpMessage::Request).map_err(|e| {
						Error::McpMessageDeserialization {
							type_name: "McpRequest",
							source: e,
						}
					})
				} else {
					// Try deserialize as McpNotification (no id or null id)
					McpNotification::deserialize(value).map(McpMessage::Notification).map_err(|e| {
						Error::McpMessageDeserialization {
							type_name: "McpNotification",
							source: e,
						}
					})
				}
			} else {
				Err(Error::McpMessageInvalidStructure(
					"Missing 'result', 'error', or 'method' field".to_string(),
				))
			}
		} else {
			Err(Error::McpMessageNotAnObject)
		}
	}
}

impl std::str::FromStr for McpMessage {
	type Err = Error;

	fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
		let value = serde_json::from_str(value).map_err(|e| Error::McpMessageDeserialization {
			type_name: "McpMessage",
			source: e,
		})?;
		McpMessage::from_value(value)
	}
}

impl Serialize for McpMessage {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			McpMessage::Request(req) => req.serialize(serializer),
			McpMessage::Notification(notif) => notif.serialize(serializer),
			McpMessage::Response(res) => res.serialize(serializer),
			McpMessage::Error(err) => err.serialize(serializer),
		}
	}
}

impl<'de> Deserialize<'de> for McpMessage {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Step 1: Deserialize into a generic Value
		let value = Value::deserialize(deserializer)?;

		// Step 2: Use the dedicated from_value function
		McpMessage::from_value(value).map_err(serde::de::Error::custom)
	}
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mcp::{
		CallToolParams, CallToolResult, GenericMeta, IntoMcpNotification, IntoMcpRequest, MessageContent,
		ProgressToken, RequestMeta,
	};
	use rpc_router::{RpcError, RpcId};
	use serde_json::json;
	use std::collections::HashMap;
	use value_ext::JsonValueExt; // For easy value creation

	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // Renamed to avoid clash

	// region:    --- Request Tests
	#[test]
	fn test_mcp_message_request_deser() -> Result<()> {
		// -- Setup & Fixtures
		let request_json = json!({
			"jsonrpc": "2.0",
			"id": "req-123",
			"method": "tools/call",
			"params": {
				"_meta": { "progressToken": "prog-abc" },
				"name": "myTool",
				"arguments": { "arg1": 123 }
			}
		});

		// -- Exec
		let mcp_message: McpMessage = serde_json::from_value(request_json.clone())?; // Using serde_json directly
		let mcp_message_from_value = McpMessage::from_value(request_json)?; // Using from_value

		// -- Check Deser
		match mcp_message {
			McpMessage::Request(req) => {
				assert_eq!(req.id, RpcId::from("req-123"));
				assert_eq!(req.method, "tools/call");
				let params_value = req.params.unwrap(); // McpMessage<Value> -> Option<Value>
				assert_eq!(params_value.x_get_str("name")?, "myTool");
				assert!(params_value.x_contains::<Value>("_meta"), "should contain meta");
				assert!(
					params_value.x_contains::<Value>("arguments"),
					"should contain arguments"
				);
				assert_eq!(
					params_value.pointer("/arguments/arg1").ok_or("Should have arg1")?,
					&json!(123)
				);
			}
			_ => panic!("Expected McpMessage::Request"),
		}

		// -- Check from_value
		match mcp_message_from_value {
			McpMessage::Request(req) => {
				assert_eq!(req.id, RpcId::from("req-123")); // Check from_value result too
			}
			_ => panic!("Expected McpMessage::Request from from_value"),
		}

		Ok(())
	}

	// endregion: --- Request Tests

	// region:    --- Notification Tests
	#[test]
	fn test_mcp_message_notification_deser() -> Result<()> {
		// -- Setup & Fixtures
		let notif_json = json!({
			"jsonrpc": "2.0",
			"method": "notifications/initialized",
			"params": {
				"_meta": { "someInfo": "extra" }
			}
		});

		// -- Exec
		let mcp_message: McpMessage = serde_json::from_value(notif_json.clone())?;
		let mcp_message_from_value = McpMessage::from_value(notif_json)?;

		// -- Check Deser
		match mcp_message {
			McpMessage::Notification(notif) => {
				assert_eq!(notif.method, "notifications/initialized");
				let params_value = notif.params.unwrap(); // McpMessage<Value> -> Option<Value>
				assert!(params_value.x_contains::<Value>("_meta"));
				assert_eq!(
					params_value.pointer("/_meta/someInfo").ok_or("Should have someInfo")?,
					&json!("extra")
				);
			}
			_ => panic!("Expected McpMessage::Notification"),
		}

		// -- Check from_value
		match mcp_message_from_value {
			McpMessage::Notification(notif) => {
				assert_eq!(notif.method, "notifications/initialized"); // Check from_value result too
			}
			_ => panic!("Expected McpMessage::Notification from from_value"),
		}
		Ok(())
	}

	// endregion: --- Notification Tests

	// region:    --- Response Tests
	#[test]
	fn test_mcp_message_response_deser() -> Result<()> {
		// -- Setup & Fixtures
		let response_json = json!({
			"jsonrpc": "2.0",
			"id": "resp-789",
			"result": {
				"_meta": { "traceId": "trace-1" },
				"content": [ { "type": "text", "text": "Tool result text" } ],
				"isError": false
			}
		});

		// -- Exec
		let mcp_message: McpMessage = serde_json::from_value(response_json.clone())?;
		let mcp_message_from_value = McpMessage::from_value(response_json)?;

		// -- Check Deser
		match mcp_message {
			McpMessage::Response(resp) => {
				assert_eq!(resp.id, RpcId::from("resp-789"));
				let result_value = resp.result; // McpResponse<Value> -> Value
				assert!(result_value.x_contains::<Value>("_meta"));
				assert_eq!(
					result_value.pointer("/_meta/traceId").ok_or("Should have traceId")?,
					&json!("trace-1")
				);
				let content_array = result_value
					.pointer("/content")
					.ok_or("Should have content")?
					.as_array()
					.unwrap();
				assert_eq!(content_array.len(), 1);
				assert_eq!(result_value.pointer("/content/0/type").unwrap(), &json!("text"));
				assert_eq!(
					result_value.pointer("/content/0/text").unwrap(),
					&json!("Tool result text")
				);
				assert_eq!(
					result_value.pointer("/isError").ok_or("Should have isError")?,
					&json!(false)
				);
			}
			_ => panic!("Expected McpMessage::Response"),
		}

		// -- Check from_value
		match mcp_message_from_value {
			McpMessage::Response(resp) => {
				assert_eq!(resp.id, RpcId::from("resp-789")); // Check from_value result too
			}
			_ => panic!("Expected McpMessage::Response from from_value"),
		}
		Ok(())
	}

	// endregion: --- Response Tests

	// region:    --- Error Tests
	#[test]
	fn test_mcp_message_error_deser() -> Result<()> {
		// -- Setup & Fixtures
		let error_json = json!({
			"jsonrpc": "2.0",
			"id": "err-101",
			"error": {
				"code": -32601,
				"message": "Method not found",
				"data": "Method 'nonExistentMethod' does not exist."
			}
		});

		// -- Exec
		let mcp_message: McpMessage = serde_json::from_value(error_json.clone())?;
		let mcp_message_from_value = McpMessage::from_value(error_json)?;

		// -- Check Deser
		match mcp_message {
			McpMessage::Error(err) => {
				assert_eq!(err.id, RpcId::from("err-101"));
				assert_eq!(err.error.code, -32601);
				assert_eq!(err.error.message, "Method not found");
				assert_eq!(
					err.error.data,
					Some(json!("Method 'nonExistentMethod' does not exist."))
				);
			}
			_ => panic!("Expected McpMessage::Error"),
		}

		// -- Check from_value
		match mcp_message_from_value {
			McpMessage::Error(err) => {
				assert_eq!(err.id, RpcId::from("err-101")); // Check from_value result too
			}
			_ => panic!("Expected McpMessage::Error from from_value"),
		}

		Ok(())
	}

	#[test]
	fn test_mcp_message_error_ser() -> Result<()> {
		// -- Setup & Fixtures
		// Create McpError directly
		let mcp_error = McpError {
			id: RpcId::from(999),
			error: RpcError {
				code: -32602,
				message: "Invalid params".to_string(),
				data: Some(json!({"details": "Missing required parameter 'foo'"})),
			},
		};
		// Wrap it in McpMessage
		let mcp_message: McpMessage = McpMessage::Error(mcp_error);

		// -- Exec
		let serialized_value = serde_json::to_value(&mcp_message)?;

		// -- Check
		assert_eq!(serialized_value["jsonrpc"], "2.0");
		assert_eq!(serialized_value["id"], 999);
		assert!(serialized_value.get("result").is_none());
		let error_val = serialized_value.get("error").unwrap();
		assert_eq!(error_val.pointer("/code").unwrap(), &json!(-32602));
		assert_eq!(error_val.pointer("/message").unwrap(), &json!("Invalid params"));
		assert_eq!(
			error_val.pointer("/data/details").unwrap(),
			&json!("Missing required parameter 'foo'")
		);

		Ok(())
	}
	// endregion: --- Error Tests

	// region:    --- Invalid Message Tests
	#[test]
	fn test_mcp_message_invalid_json_deser() {
		// -- Setup & Fixtures
		let invalid_json_str = r#"{"jsonrpc": "2.0", "id": 1, "method": "test" "#; // Incomplete JSON

		// -- Exec & Check (serde_json)
		let result_deser: std::result::Result<McpMessage, _> = serde_json::from_str(invalid_json_str);
		assert!(result_deser.is_err());

		// -- Exec & Check (from_value) - requires parsing to Value first, which fails
		let value_result: std::result::Result<Value, _> = serde_json::from_str(invalid_json_str);
		assert!(value_result.is_err()); // Parsing to Value itself fails
	}

	#[test]
	fn test_mcp_message_not_object_deser() -> Result<()> {
		// -- Setup & Fixtures
		let not_object_json = json!(["array", "is", "not", "object"]);

		// -- Exec & Check (serde_json)
		let result_deser: std::result::Result<McpMessage, _> = serde_json::from_value(not_object_json.clone());
		assert!(result_deser.is_err());
		let err = result_deser.err().ok_or("Should have error")?;
		assert!(err.to_string().contains("McpMessageNotAnObject"));

		// -- Exec & Check (from_value)
		let result_from_value = McpMessage::from_value(not_object_json);
		assert!(result_from_value.is_err());
		assert!(matches!(result_from_value.unwrap_err(), Error::McpMessageNotAnObject));

		Ok(())
	}

	#[test]
	fn test_mcp_message_missing_fields_deser() {
		// -- Setup & Fixtures
		let missing_fields_json = json!({
			"jsonrpc": "2.0",
			"id": 1
			// Missing 'method', 'result', or 'error'
		});

		// -- Exec & Check (serde_json)
		let result_deser: std::result::Result<McpMessage, _> = serde_json::from_value(missing_fields_json.clone());
		assert!(result_deser.is_err());
		assert!(
			result_deser
				.unwrap_err()
				.to_string()
				.contains("Missing 'result', 'error', or 'method' field")
		);

		// -- Exec & Check (from_value)
		let result_from_value = McpMessage::from_value(missing_fields_json);
		assert!(result_from_value.is_err());
		assert!(matches!(
			result_from_value.unwrap_err(),
			Error::McpMessageInvalidStructure(_)
		));
	}

	#[test]
	fn test_mcp_message_both_result_error_deser() -> Result<()> {
		// -- Setup & Fixtures
		let both_fields_json = json!({
			"jsonrpc": "2.0",
			"id": 1,
			"result": "success",
			"error": {"code": -32000, "message": "error"}
		});

		// -- Exec & Check (serde_json)
		// It should try McpResponse first. McpResponse deserializer should catch the 'both result and error' issue.
		let result_deser: std::result::Result<McpMessage, _> = serde_json::from_value(both_fields_json.clone());
		assert!(result_deser.is_err());
		let err_string = result_deser.unwrap_err().to_string();
		assert!(err_string.contains("BothResultAndError")); // Check underlying rpc-router error

		// -- Exec & Check (from_value)
		let result_from_value = McpMessage::from_value(both_fields_json);
		assert!(result_from_value.is_err());
		match result_from_value.unwrap_err() {
			Error::McpMessageDeserialization { type_name, source } => {
				assert_eq!(type_name, "McpResponse");
				assert!(source.to_string().contains("BothResultAndError"));
			}
			_ => panic!("Expected McpMessageDeserialization error"),
		}

		Ok(())
	}

	#[test]
	fn test_mcp_message_request_with_null_id_deser() -> Result<()> {
		// -- Setup & Fixtures
		let req_null_id_json = json!({
			"jsonrpc": "2.0",
			"id": null, // Should be treated as notification
			"method": "someMethod",
			"params": {}
		});

		// -- Exec (serde_json)
		let mcp_message_deser: McpMessage = serde_json::from_value(req_null_id_json.clone())?;
		// -- Exec (from_value)
		let mcp_message_from_value = McpMessage::from_value(req_null_id_json)?;

		// -- Check Deser
		assert!(matches!(mcp_message_deser, McpMessage::Notification(_)));
		if let McpMessage::Notification(notif) = mcp_message_deser {
			assert_eq!(notif.method, "someMethod");
		} else {
			panic!("Expected Notification from serde_json");
		}

		// -- Check from_value
		assert!(matches!(mcp_message_from_value, McpMessage::Notification(_)));
		if let McpMessage::Notification(notif) = mcp_message_from_value {
			assert_eq!(notif.method, "someMethod");
		} else {
			panic!("Expected Notification from from_value");
		}
		Ok(())
	}
	// endregion: --- Invalid Message Tests
}

// endregion: --- Tests
