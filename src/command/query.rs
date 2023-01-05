use crate::error::CliResult;
use crate::ethereum::ethers_rs;
use crate::ethereum::ethers_rs::{rust_token1155, rust_token721};
use crate::model::Network;
use crate::open_sea::api::OrderSide;
use crate::open_sea::{api, ApiClient};
use crate::CliError;

pub async fn get_balance(network: Network) -> CliResult<()> {
    let ether = ethers_rs::get_balance(network).await?;
    println!("balance ether: {}", ether);

    Ok(())
}

pub async fn show_token_contract(network: Network) -> CliResult<()> {
    let erc721_cli = rust_token721::Client::new(network);
    println!("------------------------------------------------------------");
    println!("ERC721 info: {}", network.erc721_address());
    println!(
        "name = {}",
        erc721_cli.simple_query::<String>("name").await?
    );
    println!(
        "latestTokenId = {}",
        erc721_cli.simple_query::<u128>("latestTokenId").await?
    );
    println!(
        "totalSupply = {:?}",
        erc721_cli.simple_query::<u128>("totalSupply").await?
    );
    println!(
        "totalOwned = {:?}",
        erc721_cli.simple_query::<u128>("totalOwned").await?
    );
    println!("------------------------------------------------------------");

    let erc1155_cli = rust_token1155::Client::new(network);
    println!("------------------------------------------------------------");
    println!("ERC1155 info: {}", network.erc1155_address());
    println!(
        "name = {}",
        erc1155_cli.simple_query::<String>("name").await?
    );
    println!(
        "latestTokenId = {}",
        erc1155_cli.simple_query::<u128>("latestTokenId").await?
    );
    println!(
        "totalSupply = {:?}",
        erc1155_cli.simple_query::<u128>("totalSupply").await?
    );
    println!(
        "totalOwned = {:?}",
        erc1155_cli.simple_query::<u128>("totalOwned").await?
    );
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
