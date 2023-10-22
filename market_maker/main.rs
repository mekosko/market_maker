use uniswap::{UniswapV2Factory, UniswapV2Pool};

pub mod uniswap;

pub enum MarketMakerFactory {
	UniswapV2Factory(UniswapV2Factory),
}

pub enum MarketMakerPool {
	UniswapV2Pool(UniswapV2Pool),
}
