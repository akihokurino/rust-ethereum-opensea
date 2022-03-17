use crate::error::CliResult;
use crate::open_sea::*;

pub async fn show() -> CliResult<()> {
    let erc721_cli = erc721::CLI::new();
    let erc1155_cli = erc1155::CLI::new();

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
