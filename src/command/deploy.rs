use crate::error::CliResult;
use crate::ethereum::ethers_rs::{rust_token1155, rust_token721};
use crate::model::{Network, Schema};

pub async fn deploy_token_contract(network: Network, schema: Schema) -> CliResult<()> {
    match schema {
        Schema::ERC721 => {
            println!("{}", "deploy erc721 contract.........");
            let erc721_cli = rust_token721::Client::new(network);
            erc721_cli.deploy().await?
        }
        Schema::ERC1155 => {
            println!("{}", "deploy erc1155 contract.........");
            let erc1155_cli = rust_token1155::Client::new(network);
            erc1155_cli.deploy().await?
        }
    }

    Ok(())
}
