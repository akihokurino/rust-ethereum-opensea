extern crate core;

mod aws;
mod command;
mod error;
mod ethereum;
mod ipfs;
mod model;
mod open_sea;

use crate::command::{deploy, key, query, transaction};
use crate::error::CliError;
use crate::model::{Network, Schema};
use crate::open_sea::api::OrderSide;
use clap::{Arg, Command};
use dotenv::dotenv;
use std::str::FromStr;

const COMMAND: &str = "command";

const COMMAND_BALANCE: &str = "balance";
const COMMAND_SEND_ETH: &str = "send-eth";
const COMMAND_MAKE_METADATA: &str = "make-metadata";
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

const ARGS_NAME: &str = "name";
const ARGS_DESCRIPTION: &str = "description";
const ARGS_IMAGE_FILENAME: &str = "image-filename";
const ARGS_IMAGE_URL: &str = "image-url";
const ARGS_AMOUNT: &str = "amount";
const ARGS_NETWORK: &str = "network";
const ARGS_SCHEMA: &str = "schema";
const ARGS_CONTRACT_ADDRESS: &str = "contract-address";
const ARGS_TOKEN_ID: &str = "token-id";
const ARGS_ETHER: &str = "ether";
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
                    COMMAND_BALANCE,
                    COMMAND_SEND_ETH,
                    COMMAND_MAKE_METADATA,
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
            Arg::new(ARGS_IMAGE_URL)
                .long(ARGS_IMAGE_URL)
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
            Arg::new(ARGS_NETWORK)
                .long(ARGS_NETWORK)
                .possible_values(&["Ethereum", "Polygon"])
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
            Arg::new(ARGS_ETHER)
                .long(ARGS_ETHER)
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
    let image_url: String = matches
        .value_of(ARGS_IMAGE_URL)
        .unwrap_or_default()
        .to_string();
    let amount: u128 = matches
        .value_of(ARGS_AMOUNT)
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);
    let network: String = matches
        .value_of(ARGS_NETWORK)
        .unwrap_or("Ethereum")
        .to_string();
    let network = Network::from_str(&network).ok().unwrap();
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
    let ether: f64 = matches
        .value_of(ARGS_ETHER)
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
        COMMAND_BALANCE => query::get_balance(network).await,
        COMMAND_SEND_ETH => transaction::send_eth(network, ether, to_address).await,
        COMMAND_MAKE_METADATA => transaction::make_metadata(name, description, image_url).await,
        COMMAND_MINT => match schema {
            Schema::ERC721 => {
                transaction::mint_erc721(network, name, description, image_filename).await
            }
            Schema::ERC1155 => {
                transaction::mint_erc1155(network, name, description, image_filename, amount).await
            }
        },
        COMMAND_TOKEN_INFO => query::show_token_contract(network).await,
        COMMAND_OPENSEA_ASSET_INFO => query::show_asset(contract_address, token_id).await,
        COMMAND_OPENSEA_SELL_ORDER_INFO => {
            query::show_order(contract_address, token_id, OrderSide::Sell).await
        }
        COMMAND_OPENSEA_BUY_ORDER_INFO => {
            query::show_order(contract_address, token_id, OrderSide::Buy).await
        }
        COMMAND_OPENSEA_SELL => transaction::sell(network, token_id, schema, ether).await,
        COMMAND_OPENSEA_TRANSFER => {
            transaction::transfer(network, token_id, schema, to_address).await
        }
        COMMAND_KEY_GEN => key::generate().await,
        COMMAND_SIGN => key::sign(message).await,
        COMMAND_VERIFY => key::verify(signature, message).await,
        COMMAND_DEPLOY_TOKEN => deploy::deploy_token_contract(network, schema).await,
        _ => Err(CliError::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }
}
