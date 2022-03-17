mod aws;
mod command;
mod error;
mod open_sea;

use crate::command::{create_nft, initialize};
use crate::error::CliError;
use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;

const COMMAND: &str = "command";
const COMMAND_INITIALIZE: &str = "initialize";
const COMMAND_CREATE_NFT: &str = "create-nft";

const NFT_NAME: &str = "nft-name";
const NFT_DESCRIPTION: &str = "nft-description";
const NFT_IMAGE_URL: &str = "nft-image-url";
const NFT_STATS: &str = "nft-stats";

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
                .possible_values(&[COMMAND_INITIALIZE, COMMAND_CREATE_NFT])
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
            Arg::new(NFT_STATS)
                .help("nft stats")
                .long(NFT_STATS)
                .multiple_values(true)
                .required(false)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let command = matches.value_of(COMMAND).unwrap();

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

    let result = match command {
        COMMAND_INITIALIZE => initialize::exec().await,
        COMMAND_CREATE_NFT => {
            create_nft::exec(nft_name, nft_description, nft_image_url, nft_stats).await
        }
        _ => Err(CliError::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }

    println!("success: {}", command);
}
