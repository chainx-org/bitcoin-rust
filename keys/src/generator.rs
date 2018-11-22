#[cfg(feature = "std")]
use rand::os::OsRng;
use network::Network;
use secp256k1::{PublicKey, SecretKey};
use {KeyPair, Error};

#[cfg(feature = "std")]
pub trait Generator {
	fn generate(&self) -> Result<KeyPair, Error>;
}

pub struct Random {
	network: Network
}

impl Random {
	pub fn new(network: Network) -> Self {
		Random {
			network: network,
		}
	}
}

#[cfg(feature = "std")]
impl Generator for Random {
	fn generate(&self) -> Result<KeyPair, Error> {
		let mut rng = OsRng::new().map_err(|_| Error::FailedKeyGeneration)?;
		let secret = SecretKey::random(&mut rng);
		let public = PublicKey::from_secret_key(&secret);
		Ok(KeyPair::from_keypair(secret, public, self.network))
	}
}
