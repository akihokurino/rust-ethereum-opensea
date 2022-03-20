use crate::error::CliResult;
use crate::open_sea::api::OrderSide;
use crate::open_sea::*;
use crate::CliError;

pub async fn show_contract() -> CliResult<()> {
    let erc721_cli = erc721::Client::new();
    let erc1155_cli = erc1155::Client::new();

    println!("------------------------------------------------------------");
    println!("ERC721 info: {}", erc721_cli.contract_address);
    let name = erc721_cli.get_name().await?;
    let base_url = erc721_cli.get_base_url().await?;
    let supply_num = erc721_cli.get_current_supply().await?;
    let used_names = erc721_cli.get_already_used_names().await?;
    println!("name = {}", name);
    println!("base_url = {}", base_url);
    println!("supply_num = {}", supply_num);
    println!("used_names = {:?}", used_names);
    println!("------------------------------------------------------------");

    println!("------------------------------------------------------------");
    println!("ERC1155 info: {}", erc1155_cli.contract_address);
    let name = erc1155_cli.get_name().await?;
    let base_url = erc1155_cli.get_base_url().await?;
    let supply_num = erc1155_cli.get_current_supply().await?;
    let used_names = erc1155_cli.get_already_used_names().await?;
    println!("name = {}", name);
    println!("base_url = {}", base_url);
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

pub async fn show_sell_order(contract_address: String, token_id: String) -> CliResult<()> {
    if contract_address.is_empty() || token_id.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let api_cli = ApiClient::new();
    let order = api_cli
        .get_order(api::get_order::Input {
            side: OrderSide::Sell,
            contract_address,
            token_id,
        })
        .await?;

    println!("{:?}", order);

    Ok(())
}
