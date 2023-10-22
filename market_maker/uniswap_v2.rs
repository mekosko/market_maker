use ethers_contract::{abigen, EthLogDecode};
use ethers_core::{abi, abi::RawLog, types::H160};
use serde::{Deserialize, Serialize};

abigen!(
	IUniswapV2Factory,
	"[event PairCreated(address indexed token0, address indexed token1, address pair, uint256)]";
);

type Result<T1> = std::result::Result<T1, abi::AbiError>;

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
	pub address: H160,
	pub token_0: H160,
	pub token_1: H160,
}

impl UniswapV2Pool {
	pub fn new(address: H160, token_0: H160, token_1: H160) -> Self {
		Self {
			address,
			token_0,
			token_1,
		}
	}

	pub fn new_from_event(event: &RawLog) -> Result<Self> {
		let PairCreatedFilter {
			token_0,
			token_1,
			pair,
			..
		} = PairCreatedFilter::decode_log(event)?;

		Ok(UniswapV2Pool {
			address: pair,
			token_0,
			token_1,
		})
	}
}
