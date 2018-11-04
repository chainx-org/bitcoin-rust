// Copyright 2018 Chainpool

#![cfg_attr(not(feature = "std"), no_std)]

extern crate bitcrypto as crypto;
extern crate chain;
extern crate keys;
extern crate primitives;
extern crate serialization as ser;
extern crate sr_std as rstd;

pub mod builder;
mod error;
mod flags;
mod interpreter;
mod num;
mod opcode;
pub mod script;
mod sign;
mod stack;
mod verify;

pub use primitives::{bytes, hash};

pub use self::builder::Builder;
pub use self::error::Error;
pub use self::flags::VerificationFlags;
pub use self::interpreter::{eval_script, verify_script};
pub use self::opcode::Opcode;
pub use self::num::Num;
pub use self::script::{Script, ScriptType, ScriptAddress, ScriptWitness, is_witness_commitment_script};
pub use self::sign::{TransactionInputSigner, UnsignedTransactionInput, SignatureVersion};
pub use self::stack::Stack;
pub use self::verify::{SignatureChecker, NoopSignatureChecker, TransactionSignatureChecker};

