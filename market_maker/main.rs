use uniswap_v2::{UniswapV2Factory, UniswapV2Pool};

pub mod uniswap_v2;

pub enum MarketMakerFactory {
	UniswapV2Factory(UniswapV2Factory),
}

pub enum MarketMakerPool {
	UniswapV2Pool(UniswapV2Pool),
}
