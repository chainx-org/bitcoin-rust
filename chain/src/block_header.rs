// Copyright 2018 Chainpool

#[cfg(feature = "std")]
use hex::FromHex;
use ser::{serialize, deserialize, Serializable, Stream, Reader, Deserializable};
use crypto::dhash256;
use compact::Compact;
use hash::H256;
use primitives::io;
use rstd::result::Result;
use rstd::prelude::Vec;

#[derive(PartialEq, Clone, Eq, Default)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_header_hash: H256,
    pub merkle_root_hash: H256,
    pub time: u32,
    pub bits: Compact,
    pub nonce: u32,
}

#[cfg(feature = "std")]
impl serde::Serialize for BlockHeader {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let value = serialize::<BlockHeader>(&self).take();
        serde_bytes::serialize(&value, serializer)
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for BlockHeader {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let value: Vec<u8> = serde_bytes::deserialize(deserializer).unwrap();
        if let Ok(header) = deserialize(Reader::new(&value)) {
            Ok(header)
        } else {
            Err(serde::de::Error::custom("header is not expect"))
        }
    }
}

impl ::codec::Encode for BlockHeader {
    fn encode(&self) -> Vec<u8> {
        let value = serialize::<BlockHeader>(&self);
        value.encode()
    }
}

impl ::codec::Decode for BlockHeader {
    fn decode<I: ::codec::Input>(input: &mut I) -> Option<Self> {
        let value: Vec<u8> = ::codec::Decode::decode(input).unwrap();
        if let Ok(header) = deserialize(Reader::new(&value)) {
            Some(header)
        } else {
            None
        }
    }
}

impl Serializable for BlockHeader {
    fn serialize(&self, stream: &mut Stream) {
        stream.append(&self.version)
            .append(&self.previous_header_hash)
            .append(&self.merkle_root_hash)
            .append(&self.time)
            .append(&self.bits)
            .append(&self.nonce);
    }
}

impl Deserializable for BlockHeader {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, io::Error> where T: io::Read {
        Ok(BlockHeader {
            version: reader.read()?,
            previous_header_hash: reader.read()?,
            merkle_root_hash: reader.read()?,
            time: reader.read()?,
            bits: reader.read()?,
            nonce: reader.read()?,
        })
    }
}

impl BlockHeader {
    pub fn hash(&self) -> H256 {
        dhash256(&serialize(self))
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BlockHeader")
            .field("version", &self.version)
            .field("previous_header_hash", &self.previous_header_hash.clone().reverse())
            .field("merkle_root_hash", &self.merkle_root_hash.clone().reverse())
            .field("time", &self.time)
            .field("bits", &self.bits)
            .field("nonce", &self.nonce)
            .finish()
    }
}

#[cfg(feature = "std")]
impl From<&'static str> for BlockHeader {
    fn from(s: &'static str) -> Self {
        deserialize(&s.from_hex::<Vec<u8>>().unwrap() as &[u8]).unwrap()
    }
}

#[cfg(test)]
mod tests {
	use ser::{Reader, Error as ReaderError, Stream};
	use super::BlockHeader;

	#[test]
	fn test_block_header_stream() {
		let block_header = BlockHeader {
			version: 1,
			previous_header_hash: [2; 32].into(),
			merkle_root_hash: [3; 32].into(),
			time: 4,
			bits: 5.into(),
			nonce: 6,
		};

		let mut stream = Stream::default();
		stream.append(&block_header);

		let expected = vec![
			1, 0, 0, 0,
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			4, 0, 0, 0,
			5, 0, 0, 0,
			6, 0, 0, 0,
		].into();

		assert_eq!(stream.out(), expected);
	}

	#[test]
	fn test_block_header_reader() {
		let buffer = vec![
			1, 0, 0, 0,
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			4, 0, 0, 0,
			5, 0, 0, 0,
			6, 0, 0, 0,
		];

		let mut reader = Reader::new(&buffer);

		let expected = BlockHeader {
			version: 1,
			previous_header_hash: [2; 32].into(),
			merkle_root_hash: [3; 32].into(),
			time: 4,
			bits: 5.into(),
			nonce: 6,
		};

		assert_eq!(expected, reader.read().unwrap());
		//assert_eq!(ReaderError::UnexpectedEnd, reader.read::<BlockHeader>().unwrap_err());
	}
    //use ser::{Reader, Error as ReaderError, Stream};
    //use super::BlockHeader;
/*
    #[test]
    fn test_block_header_stream() {
        let block_header = BlockHeader {
            version: 1,
            previous_header_hash: [2; 32].into(),
            merkle_root_hash: [3; 32].into(),
            time: 4,
            bits: 5.into(),
            nonce: 6,
        };

        let mut stream = Stream::default();
        stream.append(&block_header);

        let expected = vec![
            1, 0, 0, 0,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 0, 0, 0,
            5, 0, 0, 0,
            6, 0, 0, 0,
        ].into();

        assert_eq!(stream.out(), expected);
    }

    #[test]
    fn test_block_header_reader() {
        let buffer = vec![
            1, 0, 0, 0,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 0, 0, 0,
            5, 0, 0, 0,
            6, 0, 0, 0,
        ];

        let mut reader = Reader::new(&buffer);

        let expected = BlockHeader {
            version: 1,
            previous_header_hash: [2; 32].into(),
            merkle_root_hash: [3; 32].into(),
            time: 4,
            bits: 5.into(),
            nonce: 6,
        };

        assert_eq!(expected, reader.read().unwrap());
        assert_eq!(ReaderError::UnexpectedEnd, reader.read::<BlockHeader>().unwrap_err());
    }
*/
}
