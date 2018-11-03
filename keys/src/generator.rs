
#[cfg(feature = "std")]
use rand::os::OsRng;
use network::Network;
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
		let context = &secp256k1::Secp256k1::new();
		let mut rng = try!(OsRng::new().map_err(|_| Error::FailedKeyGeneration));
		let (secret, public) = try!(context.generate_keypair(&mut rng));
		Ok(KeyPair::from_keypair(secret, public, self.network))
	}
}
