use ethers_contract::EthEvent;
use ethers_core::{
	abi::{AbiError, RawLog},
	types::{Address, H160, U256},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, EthEvent)]
#[ethevent(abi = "PairCreated(address,address,address,uint256)")]
pub struct PairCreatedEvent {
	#[ethevent(indexed)]
	pub token_0: Address,
	#[ethevent(indexed)]
	pub token_1: Address,
	pub address: Address,
	pub count: U256,
}

type Result<T1> = std::result::Result<T1, AbiError>;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UniswapV2Factory {
	pub address: H160,
	pub block: u64,
}

impl UniswapV2Factory {
	pub fn new(address: H160, block: u64) -> Self {
		Self { address, block }
	}
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UniswapV2Pool {
	pub token_0: H160,
	pub token_1: H160,
	pub address: H160,
}

impl UniswapV2Pool {
	pub fn new(token_0: H160, token_1: H160, address: H160) -> Self {
		Self {
			token_0,
			token_1,
			address,
		}
	}

	pub fn new_from_event(event: &RawLog) -> Result<Self> {
		let PairCreatedEvent {
			token_0,
			token_1,
			address,
			..
		} = PairCreatedEvent::decode_log(event)?;

		Ok(UniswapV2Pool {
			token_0,
			token_1,
			address,
		})
	}
}
