//! A standalone listener that peers with the Ethereum p2p network and surfaces pending
//! transactions gossiped across it.

use reth_transaction_pool::{
    EthPooledTransaction, NewTransactionEvent, TransactionListenerKind, TransactionPool,
};
use tokio_stream::wrappers::ReceiverStream;

use crate::reth::{RethP2P, RethP2PConfig};

/// Transactions stream produced by reth's network.
pub type TransactionStream =
    tokio_stream::wrappers::ReceiverStream<NewTransactionEvent<EthPooledTransaction>>;

/// Ethereum P2P .
pub struct PearTower {
    reth: RethP2P,
}

/// Configurator for [`PearTower`].
#[derive(bon::Builder)]
pub struct PearTowerConfig {
    /// Config for reth P2P.
    pub reth: RethP2PConfig,
}

impl PearTower {
    /// Returns a reference for [`RethP2P`].
    pub fn reth(&self) -> &RethP2P {
        &self.reth
    }

    /// Returns a transactions stream from [`RethP2P`] network.
    pub fn transaction_stream(&self) -> TransactionStream {
        ReceiverStream::new(
            self.reth.pool.new_transactions_listener_for(TransactionListenerKind::All),
        )
    }
}
