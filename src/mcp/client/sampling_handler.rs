use crate::mcp::{CreateMessageParams, Result, SamplingMessage};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait SamplingHandlerAsyncFn: Send + Sync {
	fn exec_fn(
		&self,
		create_message_params: CreateMessageParams,
	) -> Pin<Box<dyn Future<Output = Result<SamplingMessage>> + Send>>;
}

impl std::fmt::Debug for dyn SamplingHandlerAsyncFn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SamplingHandlerAsyncFn")
	}
}

// region:    --- Adapter for generic async functions

struct GenericFnAdapter<FN, FUT>
where
	FN: FnOnce(CreateMessageParams) -> FUT + Send + Sync + Clone + 'static,
	FUT: Future<Output = Result<SamplingMessage>> + Send + 'static,
{
	f: FN,
	_phantom: std::marker::PhantomData<fn() -> FUT>,
}

impl<FN, FUT> SamplingHandlerAsyncFn for GenericFnAdapter<FN, FUT>
where
	FN: FnOnce(CreateMessageParams) -> FUT + Send + Sync + Clone + 'static,
	FUT: Future<Output = Result<SamplingMessage>> + Send + 'static,
{
	fn exec_fn(
		&self,
		create_message_params: CreateMessageParams,
	) -> Pin<Box<dyn Future<Output = Result<SamplingMessage>> + Send>> {
		Box::pin((self.f.clone())(create_message_params))
	}
}

// endregion: --- Adapter for generic async functions

// region:    --- IntoSamplingHandlerAsyncFn Trait ---

pub trait IntoSamplingHandlerAsyncFn {
	fn into_sampling_handler(self) -> Arc<Box<dyn SamplingHandlerAsyncFn>>;
}

impl IntoSamplingHandlerAsyncFn for Arc<Box<dyn SamplingHandlerAsyncFn>> {
	fn into_sampling_handler(self) -> Arc<Box<dyn SamplingHandlerAsyncFn>> {
		self
	}
}

impl<F, Fut> IntoSamplingHandlerAsyncFn for F
where
	F: FnOnce(CreateMessageParams) -> Fut + Send + Sync + Clone + 'static,
	Fut: Future<Output = Result<SamplingMessage>> + Send + 'static,
{
	fn into_sampling_handler(self) -> Arc<Box<dyn SamplingHandlerAsyncFn>> {
		let adapter = GenericFnAdapter {
			f: self,
			_phantom: std::marker::PhantomData,
		};
		Arc::new(Box::new(adapter))
	}
}

// endregion: --- IntoSamplingHandlerAsyncFn Trait ---
