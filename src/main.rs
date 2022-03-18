mod aws;
mod command;
mod error;
mod open_sea;

use crate::command::{info, init, mint};
use crate::error::CliError;
use clap::{Arg, Command};
use dotenv::dotenv;

const COMMAND: &str = "command";
const COMMAND_INIT: &str = "init";
const COMMAND_MINT: &str = "mint";
const COMMAND_INFO: &str = "info";

const NFT_NAME: &str = "nft-name";
const NFT_DESCRIPTION: &str = "nft-description";
const NFT_IMAGE_URL: &str = "nft-image-url";
const NFT_AMOUNT: &str = "nft-amount";
const NFT_STATS: &str = "nft-stats";
const NFT_SCHEMA: &str = "nft-schema";
const NFT_SCHEMA_ERC721: &str = "erc721";
const NFT_SCHEMA_ERC1155: &str = "erc1155";

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let app = Command::new("rust-opensea")
        .version("0.1.0")
        .author("akiho <aki030402@mail.com>")
        .about("OpenSea CLI")
        .arg(
            Arg::new(COMMAND)
                .help("exec command name")
                .long(COMMAND)
                .possible_values(&[COMMAND_INIT, COMMAND_MINT, COMMAND_INFO])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_NAME)
                .help("nft name")
                .long(NFT_NAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_DESCRIPTION)
                .help("nft description")
                .long(NFT_DESCRIPTION)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_IMAGE_URL)
                .help("nft image url")
                .long(NFT_IMAGE_URL)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_AMOUNT)
                .help("nft amount")
                .long(NFT_AMOUNT)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_STATS)
                .help("nft stats")
                .long(NFT_STATS)
                .multiple_values(true)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(NFT_SCHEMA)
                .help("nft schema")
                .long(NFT_SCHEMA)
                .possible_values(&[NFT_SCHEMA_ERC721, NFT_SCHEMA_ERC1155])
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

    let result = match matches.value_of(COMMAND).unwrap() {
        COMMAND_INIT => init::exec().await,
        COMMAND_MINT => match nft_schema.as_str() {
            NFT_SCHEMA_ERC721 => {
                mint::erc721(nft_name, nft_description, nft_image_url, nft_stats).await
            }
            NFT_SCHEMA_ERC1155 => {
                mint::erc1155(
                    nft_name,
                    nft_description,
                    nft_image_url,
                    nft_amount,
                    nft_stats,
                )
                .await
            }
            _ => Err(CliError::Internal("unknown schema".to_string())),
        },
        COMMAND_INFO => info::show().await,
        _ => Err(CliError::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }
}
