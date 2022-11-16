use crate::error::CliResult;
use crate::ethereum::ethers_rs::{hello, sample_oracle};
use crate::ethereum::rust_web3::{rust_token1155, rust_token721};
use crate::open_sea::api::OrderSide;
use crate::open_sea::{api, ApiClient};
use crate::CliError;
use ethers::abi::Address;
use std::env;

pub async fn show_token_contract() -> CliResult<()> {
    let erc721_contract_address = env::var("ERC721_ADDRESS").expect("ERC721_ADDRESS must be set");
    let erc1155_contract_address = env::var("ERC1155_ADDRESS").expect("ERC721_ADDRESS must be set");

    let erc721_cli = rust_token721::Client::new();
    let erc1155_cli = rust_token1155::Client::new();

    println!("------------------------------------------------------------");
    println!("ERC721 info: {}", erc721_contract_address);
    let name = erc721_cli.get_name().await?;
    let supply_num = erc721_cli.get_current_supply().await?;
    let used_names = erc721_cli.get_already_used_names().await?;
    println!("contract_name = {}", name);
    println!("supply_num = {}", supply_num);
    println!("used_names = {:?}", used_names);
    println!("------------------------------------------------------------");

    println!("------------------------------------------------------------");
    println!("ERC1155 info: {}", erc1155_contract_address);
    let name = erc1155_cli.get_name().await?;
    let supply_num = erc1155_cli.get_current_supply().await?;
    let used_names = erc1155_cli.get_already_used_names().await?;
    println!("contract_name = {}", name);
    println!("supply_num = {}", supply_num);
    println!("used_names = {:?}", used_names);
    println!("------------------------------------------------------------");

    Ok(())
}

pub async fn show_asset(contract_address: String, token_id: String) -> CliResult<()> {
    if contract_address.is_empty() || token_id.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let api_cli = ApiClient::new();
    let asset = api_cli
        .get_asset(api::get_asset::Input {
            contract_address,
            token_id,
        })
        .await?;

    println!("{:?}", asset);

    Ok(())
}

pub async fn show_order(
    contract_address: String,
    token_id: String,
    side: OrderSide,
) -> CliResult<()> {
    if contract_address.is_empty() || token_id.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let api_cli = ApiClient::new();
    let order = api_cli
        .get_order(api::get_order::Input {
            side,
            contract_address,
            token_id,
        })
        .await?;

    if order.orders.len() == 0 {
        return Err(CliError::NotFound);
    }

    println!("{:?}", order.orders.first().unwrap());

    Ok(())
}

pub async fn show_sample_oracle_contract() -> CliResult<()> {
    let cli = sample_oracle::Client::new();
    cli.simple_query::<u128>("getLatestPrice").await?;
    cli.simple_query::<Address>("getChainlinkToken").await?;
    cli.simple_query::<u128>("chainlinkFee").await?;
    cli.simple_query::<ethers::abi::FixedBytes>("timeJobId")
        .await?;
    cli.simple_query::<Address>("oracleAddress").await?;

    Ok(())
}

pub async fn show_hello_contract() -> CliResult<()> {
    let cli = hello::Client::new();
    cli.simple_query::<String>("message").await?;

    Ok(())
}
