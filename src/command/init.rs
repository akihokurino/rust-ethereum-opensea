use crate::error::CliResult;
use crate::open_sea::*;

pub async fn exec() -> CliResult<()> {
    let erc721_cli = erc721::Client::new();
    let erc1155_cli = erc1155::Client::new();

    let base_url = "https://ipfs.moralis.io:2053/ipfs/";

    erc721_cli.set_base_url(base_url.to_string()).await?;
    erc1155_cli.set_base_url(base_url.to_string()).await?;

    Ok(())
}
