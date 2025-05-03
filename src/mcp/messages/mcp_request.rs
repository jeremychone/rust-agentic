use crate::mcp::{Error, Result};
use rpc_router::{RpcId, RpcRequest};
use serde::de::DeserializeOwned;
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

impl<P: Serialize + IntoMcpRequest<P>> McpRequest<P> {
	pub fn new(id: impl Into<RpcId>, params: P) -> Self {
		McpRequest {
			id: id.into(),
			method: P::METHOD.into(),
			params: Some(params),
		}
	}
}

/// Generic stringify implementation for any type that implement IntoMcpRequest
impl<P: Serialize> McpRequest<P> {
	pub fn stringify(&self) -> Result<String> {
		serde_json::to_string(&self).map_err(Error::custom_from_err)
	}
	pub fn stringify_pretty(&self) -> Result<String> {
		serde_json::to_string_pretty(&self).map_err(Error::custom_from_err)
	}
}

// region:    --- IntoRequest

pub trait IntoMcpRequest<P>: Serialize + Sized + Into<McpRequest<P>>
where
	Self::McpResult: DeserializeOwned,
{
	const METHOD: &'static str;
	type McpResult;

	fn into_mcp_request(self) -> McpRequest<P> {
		self.into()
	}
}

/// Blanket implementation for all params that implement IntoMcpRequest on themselves
impl<P: Serialize + IntoMcpRequest<P>> From<P> for McpRequest<P> {
	fn from(params: P) -> Self {
		let id = RpcId::new_uuid_v7_base58();
		McpRequest {
			id,
			method: P::METHOD.to_string(),
			params: Some(params),
		}
	}
}

/// Blanket implementation for all self `McpRequest<P>`
/// This allow to pass Params or `McpRequest<P>` in the `Client::send_request(...)`
/// e.g., `impl IntoMcpRequest<ListToolsParams>` for `McpRequest<ListToolsParams>`
impl<P> IntoMcpRequest<P> for McpRequest<P>
where
	P: IntoMcpRequest<P>,
{
	const METHOD: &'static str = P::METHOD;
	type McpResult = P::McpResult;
}

// endregion: --- IntoRequest

// region:    --- Custom De/Serialization

impl<P> Serialize for McpRequest<P>
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
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
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
