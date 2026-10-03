#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! # pear-tower
//!
//! Listens to the Ethereum devp2p network for pending transactions, emitting them as alloy rpc
//! [`Transaction`](alloy::rpc::types::Transaction)s. See [`P2PListener`].

pub mod tower;
pub use tower::{PearTower, TransactionStream};

/// Reth P2P adapters.
pub mod reth;
pub use reth::{RethP2P, RethP2PConfig};
