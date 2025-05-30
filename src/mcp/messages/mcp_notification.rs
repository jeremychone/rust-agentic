use crate::mcp::{Error, Result};
use rpc_router::RpcNotification;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct McpNotification<P = Value> {
	/// The Method of this notification
	pub method: String,

	/// The Params
	pub params: Option<P>,
}

impl<P: Serialize> McpNotification<P> {
	pub fn stringify(&self) -> Result<String> {
		serde_json::to_string(&self).map_err(Error::custom_from_err)
	}
	pub fn stringify_pretty(&self) -> Result<String> {
		serde_json::to_string_pretty(&self).map_err(Error::custom_from_err)
	}
}

// region:    --- IntoMcpNotification

pub trait IntoMcpNotification: Sized {
	const METHOD: &'static str;

	fn into_mcp_notification(self) -> McpNotification<Self> {
		self.into()
	}
}

impl<T: IntoMcpNotification> From<T> for McpNotification<T> {
	fn from(params: T) -> Self {
		McpNotification {
			method: T::METHOD.to_string(),
			params: Some(params),
		}
	}
}

// endregion: --- IntoMcpNotification

// region:    --- Custom De/Serialization

impl<P> Serialize for McpNotification<P>
where
	P: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Convert params to Option<Value> if present
		let params_value = match &self.params {
			Some(p) => Some(serde_json::to_value(p).map_err(serde::ser::Error::custom)?),
			None => None,
		};

		// Create RpcRequest
		let rpc_request = RpcNotification {
			method: self.method.clone(),
			params: params_value,
		};

		// Serialize the RpcRequest
		rpc_request.serialize(serializer)
	}
}

impl<'de, P> Deserialize<'de> for McpNotification<P>
where
	P: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Deserialize into RpcRequest first to leverage its validation
		let rpc_request = RpcNotification::deserialize(deserializer)?;

		// Convert params from Option<Value> to Option<P>
		let params = match rpc_request.params {
			Some(value) => {
				let p = P::deserialize(value).map_err(DeError::custom)?;
				Some(p)
			}
			None => None,
		};

		// Construct the Request
		Ok(McpNotification {
			method: rpc_request.method,
			params,
		})
	}
}

// endregion: --- Custom De/Serialization
