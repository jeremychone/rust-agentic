use super::Result;
use flume::{Receiver, Sender};
use tracing::error;

pub struct ClientTrx {
	pub c2s_tx: CommTx,
	pub s2c_rx: CommRx,
	pub s2c_aux_rx: CommRx,
}

pub struct TransportTrx {
	pub c2s_rx: CommRx,
	pub s2c_tx: CommTx,
	pub s2c_aux_tx: CommTx,
}

pub fn new_trx_pair() -> (ClientTrx, TransportTrx) {
	let (c2s_tx, c2s_rx) = flume::unbounded::<String>();
	let (s2c_tx, s2c_rx) = flume::unbounded::<String>();
	let (s2c_aux_tx, s2c_aux_rx) = flume::unbounded::<String>();

	let client_trx = ClientTrx {
		c2s_tx: c2s_tx.into(),
		s2c_rx: s2c_rx.into(),
		s2c_aux_rx: s2c_aux_rx.into(),
	};
	let transport_trx = TransportTrx {
		c2s_rx: c2s_rx.into(),
		s2c_tx: s2c_tx.into(),
		s2c_aux_tx: s2c_aux_tx.into(),
	};
	(client_trx, transport_trx)
}

// region:    --- CommTx

#[derive(Clone)]
pub struct CommTx {
	tx: Sender<String>,
}

impl CommTx {
	pub async fn send(&self, item: impl Into<String>) -> Result<()> {
		match self.tx.send_async(item.into()).await {
			Ok(_) => Ok(()),
			Err(err) => {
				error!("Cannot send to CommTx");
				Err(err.into())
			}
		}
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
