mod aws;
mod command;
mod error;
mod ethereum;
mod model;
mod open_sea;

use crate::command::{info, init, key, mint, transaction};
use crate::error::CliError;
use crate::model::Schema;
use crate::open_sea::api::OrderSide;
use clap::{Arg, Command};
use dotenv::dotenv;
use std::str::FromStr;

const COMMAND: &str = "command";

const COMMAND_MINT: &str = "mint";
const COMMAND_CONTRACT_INFO: &str = "contract-info";
const COMMAND_ASSET_INFO: &str = "asset-info";
const COMMAND_SELL_ORDER_INFO: &str = "sell-order-info";
const COMMAND_BUY_ORDER_INFO: &str = "buy-order-info";
const COMMAND_SELL: &str = "sell";
const COMMAND_TRANSFER: &str = "transfer";
const COMMAND_KEY_GEN: &str = "key-gen";
const COMMAND_DEPLOY_CONTRACT: &str = "deploy-contract";

const ARGS_NAME: &str = "name";
const ARGS_DESCRIPTION: &str = "description";
const ARGS_IMAGE_FILENAME: &str = "image-filename";
const ARGS_AMOUNT: &str = "amount";
const ARGS_SCHEMA: &str = "schema";
const ARGS_CONTRACT_ADDRESS: &str = "contract-address";
const ARGS_TOKEN_ID: &str = "token-id";
const ARGS_SELL_ETHER: &str = "sell-ether";
const ARGS_TO_ADDRESS: &str = "to-address";

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let app = Command::new("rust-opensea")
        .version("0.1.0")
        .author("akiho <aki030402@mail.com>")
        .about("OpenSea CLI")
        .arg(
            Arg::new(COMMAND)
                .long(COMMAND)
                .possible_values(&[
                    COMMAND_MINT,
                    COMMAND_CONTRACT_INFO,
                    COMMAND_ASSET_INFO,
                    COMMAND_SELL_ORDER_INFO,
                    COMMAND_BUY_ORDER_INFO,
                    COMMAND_SELL,
                    COMMAND_TRANSFER,
                    COMMAND_KEY_GEN,
                    COMMAND_DEPLOY_CONTRACT,
                ])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_NAME)
                .long(ARGS_NAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_DESCRIPTION)
                .long(ARGS_DESCRIPTION)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_IMAGE_FILENAME)
                .long(ARGS_IMAGE_FILENAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_AMOUNT)
                .long(ARGS_AMOUNT)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_SCHEMA)
                .long(ARGS_SCHEMA)
                .possible_values(&["ERC721", "ERC1155"])
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_CONTRACT_ADDRESS)
                .long(ARGS_CONTRACT_ADDRESS)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_TOKEN_ID)
                .long(ARGS_TOKEN_ID)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_SELL_ETHER)
                .long(ARGS_SELL_ETHER)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_TO_ADDRESS)
                .long(ARGS_TO_ADDRESS)
                .required(false)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let name: String = matches.value_of(ARGS_NAME).unwrap_or_default().to_string();
    let description: String = matches
        .value_of(ARGS_DESCRIPTION)
        .unwrap_or_default()
        .to_string();
    let image_filename: String = matches
        .value_of(ARGS_IMAGE_FILENAME)
        .unwrap_or_default()
        .to_string();
    let amount: u128 = matches
        .value_of(ARGS_AMOUNT)
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);
    let schema: String = matches
        .value_of(ARGS_SCHEMA)
        .unwrap_or("ERC721")
        .to_string();
    let schema = Schema::from_str(&schema).ok().unwrap();
    let contract_address: String = matches
        .value_of(ARGS_CONTRACT_ADDRESS)
        .unwrap_or_default()
        .to_string();
    let token_id: String = matches
        .value_of(ARGS_TOKEN_ID)
        .unwrap_or_default()
        .to_string();
    let sell_ether: f64 = matches
        .value_of(ARGS_SELL_ETHER)
        .unwrap_or_default()
        .parse()
        .unwrap_or(0.0);
    let to_address: String = matches
        .value_of(ARGS_TO_ADDRESS)
        .unwrap_or_default()
        .to_string();

    let result = match matches.value_of(COMMAND).unwrap() {
        COMMAND_DEPLOY_CONTRACT => init::deploy_contract().await,
        COMMAND_MINT => match schema {
            Schema::ERC721 => mint::erc721(name, description, image_filename).await,
            Schema::ERC1155 => mint::erc1155(name, description, image_filename, amount).await,
        },
        COMMAND_CONTRACT_INFO => info::show_contract().await,
        COMMAND_ASSET_INFO => info::show_asset(contract_address, token_id).await,
        COMMAND_SELL_ORDER_INFO => {
            info::show_order(contract_address, token_id, OrderSide::Sell).await
        }
        COMMAND_BUY_ORDER_INFO => {
            info::show_order(contract_address, token_id, OrderSide::Buy).await
        }
        COMMAND_SELL => transaction::sell(token_id, schema, sell_ether).await,
        COMMAND_TRANSFER => transaction::transfer(token_id, schema, to_address).await,
        COMMAND_KEY_GEN => key::generate().await,
        _ => Err(CliError::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }
}
