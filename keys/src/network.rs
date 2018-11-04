// Copyright 2018 Chainpool

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
