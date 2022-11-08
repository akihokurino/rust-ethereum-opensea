use crate::aws::lambda;
use crate::error::CliResult;
use crate::open_sea::*;
use crate::CliError;
use std::fs::File;
use std::io::Read;

pub async fn erc721(name: String, description: String, image_filename: String) -> CliResult<()> {
    if name.is_empty() || description.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_filename.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let mut file = File::open(format!("asset/{}", image_filename))?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let output = lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::create_metadata(
        &name,
        &description,
        "",
        base64::encode(buf),
    ))
    .await?;
    if output.ipfs_response.is_none() {
        return Err(CliError::Internal(
            "IPFSのサーバーが起動していません".to_string(),
        ));
    }
    let ipfs_hash = output.ipfs_response.unwrap().hash;

    let erc721_cli = erc721::Client::new();
    erc721_cli.mint(ipfs_hash).await?;

    Ok(())
}

pub async fn erc1155(
    name: String,
    description: String,
    image_filename: String,
    amount: u128,
) -> CliResult<()> {
    if name.is_empty() || description.is_empty() || amount <= 0 {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_filename.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let mut file = File::open(format!("asset/{}", image_filename))?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let output = lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::create_metadata(
        &name,
        &description,
        "",
        base64::encode(buf),
    ))
    .await?;
    if output.ipfs_response.is_none() {
        return Err(CliError::Internal(
            "IPFSのサーバーが起動していません".to_string(),
        ));
    }
    let ipfs_hash = output.ipfs_response.unwrap().hash;

    let erc1155_cli = erc1155::Client::new();
    erc1155_cli.mint(ipfs_hash, amount).await?;

    Ok(())
}
