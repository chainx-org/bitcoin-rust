
#[cfg(feature = "std")]
use std::fmt;
use secp256k1::Error as SecpError;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(PartialEq)]
pub enum Error {
	InvalidPublic,
	InvalidSecret,
	InvalidMessage,
	InvalidSignature,
	InvalidNetwork,
	InvalidChecksum,
	InvalidPrivate,
	InvalidAddress,
	FailedKeyGeneration,
}

#[cfg(feature = "std")]
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = match *self {
			Error::InvalidPublic => "Invalid Public",
			Error::InvalidSecret => "Invalid Secret",
			Error::InvalidMessage => "Invalid Message",
			Error::InvalidSignature => "Invalid Signature",
			Error::InvalidNetwork => "Invalid Network",
			Error::InvalidChecksum => "Invalid Checksum",
			Error::InvalidPrivate => "Invalid Private",
			Error::InvalidAddress => "Invalid Address",
			Error::FailedKeyGeneration => "Key generation failed",
		};

		msg.fmt(f)
	}
}

impl From<SecpError> for Error {
	fn from(e: SecpError) -> Self {
		match e {
			SecpError::InvalidPublicKey	=> Error::InvalidPublic,
			SecpError::InvalidSecretKey => Error::InvalidSecret,
			SecpError::InvalidMessage => Error::InvalidMessage,
			_ => Error::InvalidSignature,
		}
	}
}
