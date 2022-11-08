use std::env;

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Schema {
    ERC721,
    ERC1155,
}

impl Schema {
    pub fn address(&self) -> String {
        match self {
            Schema::ERC721 => env::var("ERC721_ADDRESS").expect("ERC721_ADDRESS must be set"),
            Schema::ERC1155 => env::var("ERC1155_ADDRESS").expect("ERC1155_ADDRESS must be set"),
        }
    }
}
