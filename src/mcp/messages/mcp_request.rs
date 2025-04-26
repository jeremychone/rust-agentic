use rpc_router::{RpcId, RpcRequest};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct McpRequest<P = Value> {
	/// The json-rpc id
	pub id: RpcId,

	/// The Method of this notification
	pub method: String,

	/// The Params
	pub params: Option<P>,
}

// region:    --- IntoRequest

pub trait IntoMcpRequest: Sized {
	const METHOD: &'static str;

	fn into_mcp_request(self) -> McpRequest<Self> {
		self.into()
	}
}

impl<T: IntoMcpRequest> From<T> for McpRequest<T> {
	fn from(params: T) -> Self {
		let id = RpcId::new_uuid_v7_base58();
		McpRequest {
			id,
			method: T::METHOD.to_string(),
			params: Some(params),
		}
	}
}

// endregion: --- IntoRequest

// region:    --- Custom De/Serialization

impl<P> Serialize for McpRequest<P>
where
	P: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Convert params to Option<Value> if present
		let params_value = match &self.params {
			Some(p) => Some(serde_json::to_value(p).map_err(serde::ser::Error::custom)?),
			None => None,
		};

		// Create RpcRequest
		let rpc_request = RpcRequest {
			id: self.id.clone(), // RpcId needs to be cloneable
			method: self.method.clone(),
			params: params_value,
		};

		// Serialize the RpcRequest
		rpc_request.serialize(serializer)
	}
}

impl<'de, P> Deserialize<'de> for McpRequest<P>
where
	P: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		// Deserialize into RpcRequest first to leverage its validation
		let rpc_request = RpcRequest::deserialize(deserializer)?;

		// Convert params from Option<Value> to Option<P>
		let params = match rpc_request.params {
			Some(value) => {
				let p = P::deserialize(value).map_err(DeError::custom)?;
				Some(p)
			}
			None => None,
		};

		// Construct the Request
		Ok(McpRequest {
			id: rpc_request.id,
			method: rpc_request.method,
			params,
		})
	}
}

// endregion: --- Custom De/Serialization
