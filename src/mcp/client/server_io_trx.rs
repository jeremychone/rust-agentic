use crate::mcp::client::Result;
use flume::{Receiver, Sender};

#[derive(Clone)]
pub(crate) struct ServerIoTrx {
	/// input to the server (use tx to send message to server)
	in_tx_rx: (Sender<String>, Receiver<String>),
	/// output from server
	out_tx_rx: (Sender<String>, Receiver<String>),
	/// err from server (not the jsonrpc error, but the other lower protocol erro)
	err_tx_rx: (Sender<String>, Receiver<String>),
}

impl Default for ServerIoTrx {
	fn default() -> Self {
		let in_tx_rx = flume::unbounded();
		let out_tx_rx = flume::unbounded();
		let err_tx_rx = flume::unbounded();

		Self {
			in_tx_rx,
			out_tx_rx,
			err_tx_rx,
		}
	}
}

impl ServerIoTrx {
	/// Use to send message to server
	pub fn in_tx(&self) -> TransportTx {
		self.in_tx_rx.0.clone().into()
	}
	/// ONLY used by transport to forward to server
	pub fn stdin_rx(&self) -> TransportRx {
		self.in_tx_rx.1.clone().into()
	}

	/// ONLY used by transport to send server message to app
	pub fn out_tx(&self) -> TransportTx {
		self.out_tx_rx.0.clone().into()
	}
	/// Use to listen to server messages
	pub fn out_rx(&self) -> TransportRx {
		self.out_tx_rx.1.clone().into()
	}
	/// ONLY used by transport to send server error to app
	pub fn stderr_tx(&self) -> TransportTx {
		self.err_tx_rx.0.clone().into()
	}
	/// Use to listen to server errors
	pub fn stderr_rx(&self) -> TransportRx {
		self.err_tx_rx.1.clone().into()
	}
}

// region:    --- TransportTx

pub struct TransportTx {
	tx: Sender<String>,
}

impl TransportTx {
	pub async fn send(&self, item: impl Into<String>) -> Result<()> {
		self.tx.send_async(item.into()).await?;
		Ok(())
	}
}

impl From<Sender<String>> for TransportTx {
	fn from(tx: Sender<String>) -> Self {
		Self { tx }
	}
}

// endregion: --- TransportTx

// region:    --- TransportRx

pub struct TransportRx {
	rx: Receiver<String>,
}

impl TransportRx {
	pub async fn recv(&self) -> Result<String> {
		let res = self.rx.recv_async().await?;
		Ok(res)
	}
}

impl From<Receiver<String>> for TransportRx {
	fn from(rx: Receiver<String>) -> Self {
		Self { rx }
	}
}

// endregion: --- TransportRx
