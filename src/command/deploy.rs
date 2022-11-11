use crate::error::CliResult;
use crate::ethereum::rust_web3::{rust_token1155, rust_token721};
use crate::model::Schema;
use std::env;

pub async fn deploy_contract(schema: Schema) -> CliResult<()> {
    match schema {
        Schema::ERC721 => {
            println!("{}", "deploy erc721 contract.........");
            let erc721_cli = rust_token721::Client::new(
                env::var("ERC721_ADDRESS").expect("ERC721_ADDRESS must be set"),
            );
            erc721_cli.deploy().await?
        }
        Schema::ERC1155 => {
            println!("{}", "deploy erc1155 contract.........");
            let erc1155_cli = rust_token1155::Client::new(
                env::var("ERC1155_ADDRESS").expect("ERC1155_ADDRESS must be set"),
            );

            erc1155_cli.deploy().await?
        }
    }

    Ok(())
}
