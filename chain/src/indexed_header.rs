// Copyright 2018 Chainpool

use rstd::cmp;
use primitives::io;
use hash::H256;
use ser::{Deserializable, Reader, Error as ReaderError};
use block_header::BlockHeader;
use read_and_hash::ReadAndHash;

#[derive(Clone)]
pub struct IndexedBlockHeader {
	pub hash: H256,
	pub raw: BlockHeader,
}

#[cfg(feature = "std")]
impl std::fmt::Debug for IndexedBlockHeader {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IndexedBlockHeader")
			.field("hash", &self.hash.clone().reverse())
			.field("raw", &self.raw)
			.finish()
	}
}

impl From<BlockHeader> for IndexedBlockHeader {
	fn from(header: BlockHeader) -> Self {
		IndexedBlockHeader {
			hash: header.hash(),
			raw: header,
		}
	}
}

impl IndexedBlockHeader {
	pub fn new(hash: H256, header: BlockHeader) -> Self {
		IndexedBlockHeader {
			hash: hash,
			raw: header,
		}
	}
}

impl cmp::PartialEq for IndexedBlockHeader {
	fn eq(&self, other: &Self) -> bool {
		self.hash == other.hash
	}
}

impl Deserializable for IndexedBlockHeader {
	fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, ReaderError> where T: io::Read {
		let data = try!(reader.read_and_hash::<BlockHeader>());
		// TODO: use len
		let header = IndexedBlockHeader {
			raw: data.data,
			hash: data.hash,
		};

		Ok(header)
	}
}
