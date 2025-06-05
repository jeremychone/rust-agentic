use crate::mcp::{CreateMessageParams, Result, SamplingMessage};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait SamplingHandlerAsyncFn: Send + Sync {
	fn exec_fn(
		&self,
		create_message_params: CreateMessageParams,
	) -> Pin<Box<dyn Future<Output = Result<SamplingMessage>> + Send>>;
	// fn clone_box(&self) -> Box<dyn SamplingHandlerAsyncFn>;
}

impl std::fmt::Debug for dyn SamplingHandlerAsyncFn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SamplingHandlerAsyncFn")
	}
}

// impl Clone for Box<dyn SamplingHandlerAsyncFn> {
// 	fn clone(&self) -> Self {
// 		self.clone()
// 	}
// }

// region:    --- Adapter for generic async functions

#[derive(Clone)]
struct GenericFnAdapter<FN, FUT>
where
	FN: Fn(CreateMessageParams) -> FUT + Send + Sync + Clone + 'static,
	FUT: Future<Output = Result<SamplingMessage>> + Send + Sync + 'static,
{
	f: FN,
	_phantom: std::marker::PhantomData<FUT>,
}

impl<FN, FUT> SamplingHandlerAsyncFn for GenericFnAdapter<FN, FUT>
where
	FN: Fn(CreateMessageParams) -> FUT + Send + Sync + Clone + 'static,
	FUT: Future<Output = Result<SamplingMessage>> + Send + Sync + 'static,
{
	fn exec_fn(
		&self,
		create_message_params: CreateMessageParams,
	) -> Pin<Box<dyn Future<Output = Result<SamplingMessage>> + Send>> {
		Box::pin((self.f)(create_message_params))
	}

	// fn clone_box(&self) -> Box<dyn SamplingHandlerAsyncFn> {
	// 	Box::new(self.clone())
	// }
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
	F: Fn(CreateMessageParams) -> Fut + Send + Sync + Clone + 'static,
	Fut: Future<Output = Result<SamplingMessage>> + Send + Sync + 'static,
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
