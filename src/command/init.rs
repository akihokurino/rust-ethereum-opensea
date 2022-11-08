use crate::error::CliResult;
use crate::ethereum::{erc1155, erc721};

pub async fn deploy_contract() -> CliResult<()> {
    let erc721_cli = erc721::Client::new();
    erc721_cli.deploy().await?;

    let erc1155_cli = erc1155::Client::new();
    erc1155_cli.deploy().await?;

    Ok(())
}
