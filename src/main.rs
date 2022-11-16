mod aws;
mod command;
mod error;
mod ethereum;
mod model;
mod open_sea;

use crate::command::{deploy, key, query, transaction};
use crate::error::CliError;
use crate::model::Schema;
use crate::open_sea::api::OrderSide;
use clap::{Arg, Command};
use dotenv::dotenv;
use std::str::FromStr;

const COMMAND: &str = "command";

const COMMAND_MINT: &str = "mint";
const COMMAND_TOKEN_INFO: &str = "token-info";
const COMMAND_OPENSEA_ASSET_INFO: &str = "opensea-asset-info";
const COMMAND_OPENSEA_SELL_ORDER_INFO: &str = "opensea-sell-order-info";
const COMMAND_OPENSEA_BUY_ORDER_INFO: &str = "opensea-buy-order-info";
const COMMAND_OPENSEA_SELL: &str = "opensea-sell";
const COMMAND_OPENSEA_TRANSFER: &str = "opensea-transfer";
const COMMAND_KEY_GEN: &str = "key-gen";
const COMMAND_SIGN: &str = "sign";
const COMMAND_VERIFY: &str = "verify";
const COMMAND_DEPLOY_TOKEN: &str = "deploy-token";

const COMMAND_SAMPLE_ORACLE_INFO: &str = "sample-oracle-info";
const COMMAND_SAMPLE_ORACLE_GET_TIME_REQUEST: &str = "sample-oracle-get-time-request";
const COMMAND_HELLO_INFO: &str = "hello-info";
const COMMAND_HELLO_SET_MESSAGE: &str = "hello-set-message";

const ARGS_NAME: &str = "name";
const ARGS_DESCRIPTION: &str = "description";
const ARGS_IMAGE_FILENAME: &str = "image-filename";
const ARGS_AMOUNT: &str = "amount";
const ARGS_SCHEMA: &str = "schema";
const ARGS_CONTRACT_ADDRESS: &str = "contract-address";
const ARGS_TOKEN_ID: &str = "token-id";
const ARGS_SELL_ETHER: &str = "sell-ethers_rs";
const ARGS_TO_ADDRESS: &str = "to-address";
const ARGS_MESSAGE: &str = "message";
const ARGS_SIGNATURE: &str = "signature";

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let app = Command::new("rust-ethereum")
        .version("0.1.0")
        .author("akiho <aki030402@mail.com>")
        .about("Ethereum OpenSea CLI")
        .arg(
            Arg::new(COMMAND)
                .long(COMMAND)
                .possible_values(&[
                    COMMAND_MINT,
                    COMMAND_TOKEN_INFO,
                    COMMAND_OPENSEA_ASSET_INFO,
                    COMMAND_OPENSEA_SELL_ORDER_INFO,
                    COMMAND_OPENSEA_BUY_ORDER_INFO,
                    COMMAND_OPENSEA_SELL,
                    COMMAND_OPENSEA_TRANSFER,
                    COMMAND_KEY_GEN,
                    COMMAND_SIGN,
                    COMMAND_VERIFY,
                    COMMAND_DEPLOY_TOKEN,
                    COMMAND_SAMPLE_ORACLE_INFO,
                    COMMAND_SAMPLE_ORACLE_GET_TIME_REQUEST,
                    COMMAND_HELLO_INFO,
                    COMMAND_HELLO_SET_MESSAGE,
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
        )
        .arg(
            Arg::new(ARGS_MESSAGE)
                .long(ARGS_MESSAGE)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_SIGNATURE)
                .long(ARGS_SIGNATURE)
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
    let message: String = matches
        .value_of(ARGS_MESSAGE)
        .unwrap_or_default()
        .to_string();
    let signature: String = matches
        .value_of(ARGS_SIGNATURE)
        .unwrap_or_default()
        .to_string();

    let result = match matches.value_of(COMMAND).unwrap() {
        COMMAND_MINT => match schema {
            Schema::ERC721 => transaction::mint_erc721(name, description, image_filename).await,
            Schema::ERC1155 => {
                transaction::mint_erc1155(name, description, image_filename, amount).await
            }
        },
        COMMAND_TOKEN_INFO => query::show_token_contract().await,
        COMMAND_OPENSEA_ASSET_INFO => query::show_asset(contract_address, token_id).await,
        COMMAND_OPENSEA_SELL_ORDER_INFO => {
            query::show_order(contract_address, token_id, OrderSide::Sell).await
        }
        COMMAND_OPENSEA_BUY_ORDER_INFO => {
            query::show_order(contract_address, token_id, OrderSide::Buy).await
        }
        COMMAND_OPENSEA_SELL => transaction::sell(token_id, schema, sell_ether).await,
        COMMAND_OPENSEA_TRANSFER => transaction::transfer(token_id, schema, to_address).await,
        COMMAND_KEY_GEN => key::generate().await,
        COMMAND_SIGN => key::sign(message).await,
        COMMAND_VERIFY => key::verify(signature, message).await,
        COMMAND_DEPLOY_TOKEN => deploy::deploy_token_contract(schema).await,

        COMMAND_SAMPLE_ORACLE_INFO => query::show_sample_oracle_contract().await,
        COMMAND_SAMPLE_ORACLE_GET_TIME_REQUEST => transaction::create_get_time_request().await,
        COMMAND_HELLO_INFO => query::show_hello_contract().await,
        COMMAND_HELLO_SET_MESSAGE => transaction::set_hello_message(message).await,

        _ => Err(CliError::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }
}
