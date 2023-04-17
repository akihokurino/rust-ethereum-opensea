use std::env;

pub mod unit;

pub const GAS_LIMIT: i64 = 8000000;
pub const GAS_PRICE: i64 = 25000000000; // 40000000000

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum TargetContract {
    RustToken721,
    RustToken1155,
    RustSbt721,
    RevealToken721,
}

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Network {
    Ethereum,
    Polygon,
    Avalanche,
}

impl Network {
    pub fn chain_url(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set"),
            Network::Polygon => env::var("POLYGON_URL").expect("POLYGON_URL must be set"),
            Network::Avalanche => env::var("AVALANCHE_URL").expect("AVALANCHE_URL must be set"),
        }
    }

    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Ethereum => env::var("ETHEREUM_CHAIN_ID")
                .expect("ETHEREUM_CHAIN_ID must be set")
                .parse::<u64>()
                .unwrap(),
            Network::Polygon => env::var("POLYGON_CHAIN_ID")
                .expect("POLYGON_CHAIN_ID must be set")
                .parse::<u64>()
                .unwrap(),
            Network::Avalanche => env::var("AVALANCHE_CHAIN_ID")
                .expect("AVALANCHE_CHAIN_ID must be set")
                .parse::<u64>()
                .unwrap(),
        }
    }

    pub fn rust_token_721_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_RUST_TOKEN_721_ADDRESS")
                .expect("ETHEREUM_RUST_TOKEN_721_ADDRESS must be set"),
            Network::Polygon => env::var("POLYGON_RUST_TOKEN_721_ADDRESS")
                .expect("POLYGON_RUST_TOKEN_721_ADDRESS must be set"),
            Network::Avalanche => env::var("AVALANCHE_RUST_TOKEN_721_ADDRESS")
                .expect("AVALANCHE_RUST_TOKEN_721_ADDRESS must be set"),
        }
    }

    pub fn rust_token_1155_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_RUST_TOKEN_1155_ADDRESS")
                .expect("ETHEREUM_RUST_TOKEN_1155_ADDRESS must be set"),
            Network::Polygon => env::var("POLYGON_RUST_TOKEN_1155_ADDRESS")
                .expect("POLYGON_RUST_TOKEN_1155_ADDRESS must be set"),
            Network::Avalanche => env::var("AVALANCHE_RUST_TOKEN_1155_ADDRESS")
                .expect("AVALANCHE_RUST_TOKEN_1155_ADDRESS must be set"),
        }
    }

    pub fn reveal_token_721_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_REVEAL_TOKEN_721_ADDRESS")
                .expect("ETHEREUM_REVEAL_TOKEN_721_ADDRESS must be set"),
            Network::Polygon => unimplemented!(),
            Network::Avalanche => unimplemented!(),
        }
    }

    pub fn rust_sbt_721_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_RUST_SBT_721_ADDRESS")
                .expect("ETHEREUM_RUST_SBT_721_ADDRESS must be set"),
            Network::Polygon => unimplemented!(),
            Network::Avalanche => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Schema {
    ERC721,
    ERC1155,
}
