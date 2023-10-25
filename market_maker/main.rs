use uniswap::{UniswapV2Factory, UniswapV2Pool};

pub mod uniswap;

#[derive(Clone, Debug)]
pub enum MarketMakerFactory {
	UniswapV2Factory(UniswapV2Factory),
}

#[derive(Clone, Debug)]
pub enum MarketMakerPool {
	UniswapV2Pool(UniswapV2Pool),
}
