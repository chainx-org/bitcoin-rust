// Copyright 2018 Chainpool
use ser::{serialize, deserialize, Serializable, Stream, Reader, Deserializable};
use primitives::io;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(PartialEq, Clone, Copy, Encode, Decode)]
pub enum Network {
	Mainnet = 0,
	Testnet = 1,
}

impl Default for Network {
	fn default() -> Network {
		Network::Mainnet
	}
}

impl Network {
	pub fn from_u32(v: u32) -> Option<Self> {
		match v {
			0 => Some(Network::Mainnet),
			1 => Some(Network::Testnet),
			_ => None
		}
	}
}

impl Serializable for Network {
	fn serialize(&self, stream: &mut Stream) {
		match *self{
			Network::Mainnet => stream.append(&Network::Mainnet),
			Network::Testnet => stream.append(&Network::Testnet),
		};
	}
}

impl Deserializable for Network {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error> where T: io::Read {
		let t: u32 = try!(reader.read());
		Network::from_u32(t).ok_or(io::ErrorKind::MalformedData)
	}
}
