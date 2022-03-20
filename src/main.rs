mod aws;
mod command;
mod error;
mod open_sea;

use crate::command::{info, init, mint, transaction};
use crate::error::CliError;
use crate::open_sea::api::OrderSide;
use clap::{Arg, Command};
use dotenv::dotenv;

const COMMAND: &str = "command";
const COMMAND_INIT: &str = "init";
const COMMAND_MINT: &str = "mint";
const COMMAND_CONTRACT_INFO: &str = "contract-info";
const COMMAND_ASSET_INFO: &str = "asset-info";
const COMMAND_SELL_ORDER_INFO: &str = "sell-order-info";
const COMMAND_BUY_ORDER_INFO: &str = "buy-order-info";
const COMMAND_BUY: &str = "buy";

const NFT_NAME: &str = "nft-name";
const NFT_DESCRIPTION: &str = "nft-description";
const NFT_IMAGE_URL: &str = "nft-image-url";
const NFT_IMAGE_FILENAME: &str = "nft-image-filename";
const NFT_AMOUNT: &str = "nft-amount";
const NFT_STATS: &str = "nft-stats";
const NFT_SCHEMA: &str = "nft-schema";
const NFT_SCHEMA_ERC721: &str = "erc721";
const NFT_SCHEMA_ERC1155: &str = "erc1155";
const NFT_CONTRACT_ADDRESS: &str = "contract-address";
const NFT_TOKEN_ID: &str = "token-id";

const TEST_API_BASE_URL: &str = "https://testnets-api.opensea.io";

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
                    COMMAND_INIT,
                    COMMAND_MINT,
                    COMMAND_CONTRACT_INFO,
                    COMMAND_ASSET_INFO,
                    COMMAND_SELL_ORDER_INFO,
                    COMMAND_BUY_ORDER_INFO,
                    COMMAND_BUY,
                ])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_NAME)
                .long(NFT_NAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_DESCRIPTION)
                .long(NFT_DESCRIPTION)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_IMAGE_URL)
                .long(NFT_IMAGE_URL)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_IMAGE_FILENAME)
                .long(NFT_IMAGE_FILENAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_AMOUNT)
                .long(NFT_AMOUNT)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_STATS)
                .long(NFT_STATS)
                .multiple_values(true)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_SCHEMA)
                .long(NFT_SCHEMA)
                .possible_values(&[NFT_SCHEMA_ERC721, NFT_SCHEMA_ERC1155])
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_CONTRACT_ADDRESS)
                .long(NFT_CONTRACT_ADDRESS)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_TOKEN_ID)
                .long(NFT_TOKEN_ID)
                .required(false)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let nft_name: String = matches.value_of(NFT_NAME).unwrap_or_default().to_string();
    let nft_description: String = matches
        .value_of(NFT_DESCRIPTION)
        .unwrap_or_default()
        .to_string();
    let nft_image_url: String = matches
        .value_of(NFT_IMAGE_URL)
        .unwrap_or_default()
        .to_string();
    let nft_image_filename: String = matches
        .value_of(NFT_IMAGE_FILENAME)
        .unwrap_or_default()
        .to_string();
    let nft_stats: Vec<_> = matches.values_of(NFT_STATS).unwrap_or_default().collect();
    let nft_stats = nft_stats
        .iter()
        .map(|val| {
            let splited: Vec<&str> = val.split('=').collect();
            (splited[0].to_string(), splited[1].to_string())
        })
        .collect::<Vec<(String, String)>>();
    let nft_amount: u128 = matches
        .value_of(NFT_AMOUNT)
        .unwrap_or_default()
        .parse()
        .unwrap_or(0);
    let nft_schema: String = matches
        .value_of(NFT_SCHEMA)
        .unwrap_or(NFT_SCHEMA_ERC721)
        .to_string();
    let nft_contract_address: String = matches
        .value_of(NFT_CONTRACT_ADDRESS)
        .unwrap_or_default()
        .to_string();
    let nft_token_id: String = matches
        .value_of(NFT_TOKEN_ID)
        .unwrap_or_default()
        .to_string();

    let result = match matches.value_of(COMMAND).unwrap() {
        COMMAND_INIT => init::exec().await,
        COMMAND_MINT => match nft_schema.as_str() {
            NFT_SCHEMA_ERC721 => {
                mint::erc721(
                    nft_name,
                    nft_description,
                    nft_image_url,
                    nft_image_filename,
                    nft_stats,
                )
                .await
            }
            NFT_SCHEMA_ERC1155 => {
                mint::erc1155(
                    nft_name,
                    nft_description,
                    nft_image_url,
                    nft_image_filename,
                    nft_amount,
                    nft_stats,
                )
                .await
            }
            _ => Err(CliError::Internal("unknown schema".to_string())),
        },
        COMMAND_CONTRACT_INFO => info::show_contract().await,
        COMMAND_ASSET_INFO => info::show_asset(nft_contract_address, nft_token_id).await,
        COMMAND_SELL_ORDER_INFO => {
            info::show_order(nft_contract_address, nft_token_id, OrderSide::Sell).await
        }
        COMMAND_BUY_ORDER_INFO => {
            info::show_order(nft_contract_address, nft_token_id, OrderSide::Buy).await
        }
        COMMAND_BUY => transaction::buy(nft_contract_address, nft_token_id).await,
        _ => Err(CliError::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }
}
