use crate::aws::s3;
use crate::error::CliResult;
use crate::open_sea::*;

pub async fn exec() -> CliResult<()> {
    let s3_cli = s3::CLI::new();
    let erc721_cli = erc721::CLI::new();
    let erc1155_cli = erc1155::CLI::new();

    let base_url = s3_cli.create_bucket().await?;
    println!("new base url: {}", base_url);

    erc721_cli.set_base_url(base_url.clone()).await?;
    erc1155_cli.set_base_url(base_url.clone()).await?;

    Ok(())
}
