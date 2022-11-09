use crate::error::CliResult;
use crate::ethereum::{erc1155, erc721};
use crate::model::Schema;

pub async fn deploy_contract(schema: Schema) -> CliResult<()> {
    match schema {
        Schema::ERC721 => {
            println!("{}", "deploy erc721 contract.........");
            let erc721_cli = erc721::Client::new();
            erc721_cli.deploy().await?
        }
        Schema::ERC1155 => {
            println!("{}", "deploy erc1155 contract.........");
            let erc1155_cli = erc1155::Client::new();
            erc1155_cli.deploy().await?
        }
    }

    Ok(())
}
