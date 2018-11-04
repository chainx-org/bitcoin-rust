//! Bitcoin keys.

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
extern crate rand;
#[cfg(feature = "std")]
extern crate rustc_hex as hex;
#[cfg(feature = "std")]
extern crate base58;
extern crate secp256k1;
extern crate bitcrypto as crypto;
extern crate primitives;
extern crate sr_std as rstd;
#[macro_use]
extern crate parity_codec_derive;
extern crate parity_codec as codec;

pub mod generator;
mod address;
mod display;
mod keypair;
mod error;
mod network;
mod private;
mod public;
mod signature;

pub use primitives::{hash, bytes};

pub use address::{Type, Address};
pub use display::DisplayLayout;
pub use keypair::KeyPair;
pub use error::Error;
pub use private::Private;
pub use public::Public;
pub use signature::{Signature, CompactSignature};
pub use network::Network;

use hash::{H160, H256};

/// 20 bytes long hash derived from public `ripemd160(sha256(public))`
pub type AddressHash = H160;
/// 32 bytes long secret key
pub type Secret = H256;
/// 32 bytes long signable message
pub type Message = H256;

