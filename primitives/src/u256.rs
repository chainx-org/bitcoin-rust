// Copyright 2018 Chainpool

use rstd::prelude::Vec;
#[cfg(not(feature="std"))]
use rstd::alloc::prelude::ToOwned;

construct_uint!(U256, 4);

impl ::codec::Encode for U256 {
	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		let mut bytes = [0u8; 4 * 8];
		self.to_little_endian(&mut bytes);
		bytes.using_encoded(f)
	}
}

impl ::codec::Decode for U256 {
	fn decode<I: ::codec::Input>(input: &mut I) -> Option<Self> {
		<[u8; 4 * 8] as ::codec::Decode>::decode(input)
			.map(|b| U256::from_little_endian(&b))
	}
}

