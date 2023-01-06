use std::env;

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Schema {
    ERC721,
    ERC1155,
}

impl Schema {
    pub fn address(&self, network: Network) -> String {
        match self {
            Schema::ERC721 => network.rust_token721_address(),
            Schema::ERC1155 => network.rust_token1155_address(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Network {
    Ethereum,
    Polygon,
}

impl Network {
    pub fn chain_url(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set"),
            Network::Polygon => env::var("POLYGON_URL").expect("POLYGON_URL must be set"),
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
        }
    }

    pub fn rust_token721_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_RUST_TOKEN_721_ADDRESS")
                .expect("ETHEREUM_RUST_TOKEN_721_ADDRESS must be set"),
            Network::Polygon => env::var("POLYGON_RUST_TOKEN_721_ADDRESS")
                .expect("POLYGON_RUST_TOKEN_721_ADDRESS must be set"),
        }
    }

    pub fn rust_token1155_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_RUST_TOKEN_1155_ADDRESS")
                .expect("ETHEREUM_RUST_TOKEN_1155_ADDRESS must be set"),
            Network::Polygon => env::var("POLYGON_RUST_TOKEN_1155_ADDRESS")
                .expect("POLYGON_RUST_TOKEN_1155_ADDRESS must be set"),
        }
    }

    pub fn reveal_token721_address(&self) -> String {
        match self {
            Network::Ethereum => env::var("ETHEREUM_REVEAL_TOKEN_721_ADDRESS")
                .expect("ETHEREUM_REVEAL_TOKEN_721_ADDRESS must be set"),
            Network::Polygon => unimplemented!(),
        }
    }
}
