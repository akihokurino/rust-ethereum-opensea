use crate::error::CliResult;
use crate::ethereum::ethers_rs;
use crate::ethereum::ethers_rs::{reveal_token721, rust_token1155, rust_token721};
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
    let rust_token721_cli = rust_token721::Client::new(network);
    println!("------------------------------------------------------------");
    println!("RustToken721 info: {}", network.rust_token721_address());
    println!(
        "name = {}",
        rust_token721_cli.simple_query::<String>("name").await?
    );
    println!(
        "latestTokenId = {}",
        rust_token721_cli
            .simple_query::<u128>("latestTokenId")
            .await?
    );
    println!(
        "totalSupply = {:?}",
        rust_token721_cli
            .simple_query::<u128>("totalSupply")
            .await?
    );
    println!(
        "totalOwned = {:?}",
        rust_token721_cli.simple_query::<u128>("totalOwned").await?
    );
    println!("------------------------------------------------------------");

    let rust_token1155_cli = rust_token1155::Client::new(network);
    println!("------------------------------------------------------------");
    println!("RustToken1155 info: {}", network.rust_token1155_address());
    println!(
        "name = {}",
        rust_token1155_cli.simple_query::<String>("name").await?
    );
    println!(
        "latestTokenId = {}",
        rust_token1155_cli
            .simple_query::<u128>("latestTokenId")
            .await?
    );
    println!(
        "totalSupply = {:?}",
        rust_token1155_cli
            .simple_query::<u128>("totalSupply")
            .await?
    );
    println!(
        "totalOwned = {:?}",
        rust_token1155_cli
            .simple_query::<u128>("totalOwned")
            .await?
    );
    println!("------------------------------------------------------------");

    if network == Network::Ethereum {
        let reveal_token721_cli = reveal_token721::Client::new(network);
        println!("------------------------------------------------------------");
        println!("RevealToken721 info: {}", network.reveal_token721_address());
        println!(
            "name = {}",
            reveal_token721_cli.simple_query::<String>("name").await?
        );
        println!(
            "totalSupply = {:?}",
            reveal_token721_cli
                .simple_query::<u128>("totalSupply")
                .await?
        );
        println!(
            "getCurrentHour = {}",
            reveal_token721_cli
                .simple_query::<u128>("getCurrentHour")
                .await?
        );
        println!("------------------------------------------------------------");
    }

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
