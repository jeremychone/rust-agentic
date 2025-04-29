use crate::mcp::client::Result;
use flume::{Receiver, Sender};

pub struct ClientTrx {
	pub in_tx: CommTx,
	pub out_rx: CommRx,
	pub err_rx: CommRx,
}

pub struct TransportTrx {
	pub in_rx: CommRx,
	pub out_tx: CommTx,
	pub err_tx: CommTx,
}

pub fn new_trx_pair() -> (ClientTrx, TransportTrx) {
	let (in_tx, in_rx) = flume::unbounded::<String>();
	let (out_tx, out_rx) = flume::unbounded::<String>();
	let (rss_tx, rss_rx) = flume::unbounded::<String>();

	let client_trx = ClientTrx {
		in_tx: in_tx.into(),
		out_rx: out_rx.into(),
		err_rx: rss_rx.into(),
	};
	let transport_trx = TransportTrx {
		in_rx: in_rx.into(),
		out_tx: out_tx.into(),
		err_tx: rss_tx.into(),
	};
	(client_trx, transport_trx)
}

// region:    --- CommTx

pub struct CommTx {
	tx: Sender<String>,
}

impl CommTx {
	pub async fn send(&self, item: impl Into<String>) -> Result<()> {
		self.tx.send_async(item.into()).await?;
		Ok(())
	}
}

impl From<Sender<String>> for CommTx {
	fn from(tx: Sender<String>) -> Self {
		Self { tx }
	}
}

// endregion: --- CommTx

// region:    --- CommRx

pub struct CommRx {
	rx: Receiver<String>,
}

impl CommRx {
	pub async fn recv(&self) -> Result<String> {
		let res = self.rx.recv_async().await?;
		Ok(res)
	}
}

impl From<Receiver<String>> for CommRx {
	fn from(rx: Receiver<String>) -> Self {
		Self { rx }
	}
}

// endregion: --- CommRx
