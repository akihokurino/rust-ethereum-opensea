use crate::aws::lambda;
use crate::error::CliResult;
use crate::ethereum::ethers_rs;
use crate::ethereum::ethers_rs::{reveal_token721, rust_sbt721, rust_token1155, rust_token721};
use crate::model::{Network, Schema};
use crate::open_sea::metadata::Metadata;
use crate::{ipfs, CliError};
use bytes::Bytes;
use std::env;
use std::fs::File;
use std::io::Read;

pub async fn send_eth(network: Network, ether: f64, address: String) -> CliResult<()> {
    ethers_rs::send_eth(
        network,
        ether,
        address
            .to_owned()
            .parse::<ethers::prelude::Address>()
            .unwrap(),
    )
    .await?;

    Ok(())
}

pub async fn make_metadata_from_url(
    name: String,
    description: String,
    image_url: String,
) -> CliResult<()> {
    let ipfs = ipfs::Adapter::new();

    if name.is_empty() || description.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_url.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let metadata = Metadata::new(&name, &image_url, &description);
    let metadata = serde_json::to_string(&metadata).map_err(CliError::from)?;
    let content_hash = ipfs.upload(Bytes::from(metadata), name.clone()).await?;
    println!(
        "metadata url: {:?}",
        format!(
            "{}/ipfs/{}",
            env::var("IPFS_GATEWAY").expect("should set IPFS_GATEWAY"),
            content_hash.hash.clone()
        )
    );
    Ok(())
}

async fn make_metadata_from_file(
    name: String,
    description: String,
    image_filename: String,
) -> CliResult<String> {
    let ipfs = ipfs::Adapter::new();

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

    let content_hash = ipfs.upload(Bytes::from(buf), name.clone()).await?;
    let metadata = Metadata::new(
        &name,
        &format!(
            "{}/ipfs/{}",
            env::var("IPFS_GATEWAY").expect("should set IPFS_GATEWAY"),
            content_hash.hash.clone()
        ),
        &description,
    );
    let metadata = serde_json::to_string(&metadata).map_err(CliError::from)?;
    let content_hash = ipfs.upload(Bytes::from(metadata), name.clone()).await?;
    println!(
        "metadata url: {:?}",
        format!(
            "{}/ipfs/{}",
            env::var("IPFS_GATEWAY").expect("should set IPFS_GATEWAY"),
            content_hash.hash.clone()
        )
    );

    Ok(content_hash.hash)
}

pub async fn mint_erc721(
    network: Network,
    name: String,
    description: String,
    image_filename: String,
) -> CliResult<()> {
    let hash = make_metadata_from_file(name, description, image_filename).await?;

    println!("{}", "minting..........");
    let rust_token721_cli = rust_token721::Client::new(network);
    rust_token721_cli.mint(hash.clone()).await?;

    if network == Network::Ethereum {
        let reveal_token721_cli = reveal_token721::Client::new(network);
        reveal_token721_cli.mint(hash.clone()).await?;

        let rust_sbt721_cli = rust_sbt721::Client::new(network);
        rust_sbt721_cli.mint(hash.clone()).await?;
    }

    Ok(())
}

pub async fn mint_erc1155(
    network: Network,
    name: String,
    description: String,
    image_filename: String,
    amount: u128,
) -> CliResult<()> {
    let hash = make_metadata_from_file(name, description, image_filename).await?;

    println!("{}", "minting..........");
    let rust_token1155_cli = rust_token1155::Client::new(network);
    rust_token1155_cli.mint(hash, amount).await?;

    Ok(())
}

pub async fn sell(network: Network, token_id: String, schema: Schema, ether: f64) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::sell(
        &schema.address(network),
        &token_id,
        &schema,
        ether,
    ))
    .await?;

    Ok(())
}

pub async fn transfer(
    network: Network,
    token_id: String,
    schema: Schema,
    to_address: String,
) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::transfer(
        &schema.address(network),
        &token_id,
        &schema,
        &to_address,
    ))
    .await?;

    Ok(())
}

pub async fn update_time(network: Network) -> CliResult<()> {
    if network == Network::Ethereum {
        let reveal_token721_cli = reveal_token721::Client::new(network);
        reveal_token721_cli.update_time().await?;
    }

    Ok(())
}
