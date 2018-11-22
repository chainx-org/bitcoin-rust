#[cfg(feature = "std")]
use std::fmt;
use rstd::ops;
use secp256k1::{
	recover, verify,
	Message as SecpMessage, RecoveryId as SecpRecoveryId,
	PublicKey as SecpPublicKey, Signature as SecpSignature
};
#[cfg(feature = "std")]
use hex::ToHex;
use crypto::dhash160;
use hash::{H264, H512, H520};
use {AddressHash, Error, CompactSignature, Signature, Message};

/// Secret public key
pub enum Public {
	/// Normal version of public key
	Normal(H520),
	/// Compressed version of public key
	Compressed(H264),
}

impl Public {
	pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
		match data.len() {
			33 => {
				let mut public = H264::default();
				public.copy_from_slice(data);
				Ok(Public::Compressed(public))
			},
			65 => {
				let mut public = H520::default();
				public.copy_from_slice(data);
				Ok(Public::Normal(public))
			},
			_ => Err(Error::InvalidPublic)
		}
	}

	pub fn address_hash(&self) -> AddressHash {
	 	let public_key: &[u8] = self;
		dhash160(public_key)
	}

	pub fn verify(&self, message: &Message, signature: &Signature) -> Result<bool, Error> {
		let public = match self {
			Public::Normal(_) => {
				SecpPublicKey::parse_slice(self, None)?
			},
			Public::Compressed(_) => {
				SecpPublicKey::parse_slice(self, None)?
			},
		};
		let mut signature = SecpSignature::parse_der_lax(&**signature)?;
		signature.normalize_s();
		let message = SecpMessage::parse(&**message);
		Ok(verify(&message, &signature, &public))
	}

	pub fn recover_compact(message: &Message, signature: &CompactSignature) -> Result<Self, Error> {
		let recovery_id = (signature[0] - 27) & 3 as u8;
		let compressed = (signature[0] - 27) & 4 != 0;
		let recovery_id = SecpRecoveryId::parse(recovery_id)?;
		let mut sign = H512::default();
		sign.copy_from_slice(&signature[1..65]);
		let signature = SecpSignature::parse(&sign);
		let message = SecpMessage::parse(&**message);
		let pub_key = recover(&message, &signature, &recovery_id)?;

		let public = if compressed {
			let mut public = H264::default();
			public.copy_from_slice(&pub_key.serialize_compressed()[..]);
			Public::Compressed(public)
		} else {
			let mut public = H520::default();
			public.copy_from_slice(&pub_key.serialize()[..]);
			Public::Normal(public)
		};
		Ok(public)
	}
}

impl ops::Deref for Public {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		match *self {
			Public::Normal(ref hash) => &**hash,
			Public::Compressed(ref hash) => &**hash,
		}
	}
}

impl PartialEq for Public {
	fn eq(&self, other: &Self) -> bool {
		let s_slice: &[u8] = self;
		let o_slice: &[u8] = other;
		s_slice == o_slice
	}
}

#[cfg(feature = "std")]
impl fmt::Debug for Public {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Public::Normal(ref hash) => writeln!(f, "normal: {}", hash.to_hex::<String>()),
			Public::Compressed(ref hash) => writeln!(f, "compressed: {}", hash.to_hex::<String>()),
		}
	}
}

#[cfg(feature = "std")]
impl fmt::Display for Public {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.to_hex::<String>().fmt(f)
	}
}
