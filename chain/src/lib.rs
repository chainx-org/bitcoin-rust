// Copyright 2018 Chainpool

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate rustc_hex as hex;
#[cfg(feature = "std")]
extern crate heapsize;
extern crate primitives;
extern crate bitcrypto as crypto;
extern crate serialization as ser;
extern crate parity_codec as codec;

#[allow(unused_imports)]
#[macro_use]
extern crate sr_std as rstd;

pub mod constants;

mod block;
mod block_header;
mod merkle_root;
mod transaction;

/// `IndexedBlock` extension
mod read_and_hash;
mod indexed_block;
mod indexed_header;
mod indexed_transaction;

pub trait RepresentH256 {
	fn h256(&self) -> hash::H256;
}

pub use primitives::{hash, bytes, compact, io};

pub use block::Block;
pub use block_header::BlockHeader;
pub use merkle_root::{merkle_root, merkle_node_hash};
pub use transaction::{Transaction, TransactionInput, TransactionOutput, OutPoint};

pub use read_and_hash::{ReadAndHash, HashedData};
pub use indexed_block::IndexedBlock;
pub use indexed_header::IndexedBlockHeader;
pub use indexed_transaction::IndexedTransaction;

pub type ShortTransactionID = hash::H48;
