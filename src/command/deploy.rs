use crate::error::CliResult;
use crate::ethereum::ethers_rs::{reveal_token721, rust_token1155, rust_token721};
use crate::model::{Network, Schema};

pub async fn deploy_token_contract(network: Network, schema: Schema) -> CliResult<()> {
    match schema {
        Schema::ERC721 => {
            println!("{}", "deploy RustToken721 contract.........");
            let rust_token721_cli = rust_token721::Client::new(network);
            rust_token721_cli.deploy().await?;

            println!("{}", "deploy RevealToken721 contract.........");
            let reveal_token721_cli = reveal_token721::Client::new(network);
            reveal_token721_cli.deploy().await?
        }
        Schema::ERC1155 => {
            println!("{}", "deploy RustToken1155 contract.........");
            let rust_token1155_cli = rust_token1155::Client::new(network);
            rust_token1155_cli.deploy().await?
        }
    }

    Ok(())
}
