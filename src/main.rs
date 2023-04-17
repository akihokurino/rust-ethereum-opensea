extern crate core;

use clap::{Arg, Command};
use common::*;
use dotenv::dotenv;
use std::str::FromStr;

const COMMAND: &str = "command";

const COMMAND_BALANCE: &str = "balance";
const COMMAND_SEND_ETH: &str = "send-eth";
const COMMAND_CREATE_METADATA: &str = "create-metadata";
const COMMAND_MINT: &str = "mint";
const COMMAND_TOKEN_INFO: &str = "token-info";
const COMMAND_KEY_GEN: &str = "key-gen";
const COMMAND_SIGN: &str = "sign";
const COMMAND_VERIFY: &str = "verify";
const COMMAND_DEPLOY_TOKEN: &str = "deploy-token";
const COMMAND_UPDATE_TIME: &str = "update-time";

const ARGS_NAME: &str = "name";
const ARGS_DESCRIPTION: &str = "description";
const ARGS_IMAGE_FILENAME: &str = "image-filename";
const ARGS_IMAGE_URL: &str = "image-url";
const ARGS_AMOUNT: &str = "amount";
const ARGS_CONTENT_HASH: &str = "content-hash";
const ARGS_PACKAGE: &str = "package";
const ARGS_NETWORK: &str = "network";
const ARGS_CONTRACT: &str = "contract";
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
                    COMMAND_CREATE_METADATA,
                    COMMAND_MINT,
                    COMMAND_TOKEN_INFO,
                    COMMAND_KEY_GEN,
                    COMMAND_SIGN,
                    COMMAND_VERIFY,
                    COMMAND_DEPLOY_TOKEN,
                    COMMAND_UPDATE_TIME,
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
            Arg::new(ARGS_CONTENT_HASH)
                .long(ARGS_CONTENT_HASH)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_PACKAGE)
                .long(ARGS_PACKAGE)
                .possible_values(&["EthersRs", "RustWeb3"])
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_NETWORK)
                .long(ARGS_NETWORK)
                .possible_values(&["Ethereum", "Polygon", "Avalanche"])
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new(ARGS_CONTRACT)
                .long(ARGS_CONTRACT)
                .possible_values(&[
                    "RustToken721",
                    "RustToken1155",
                    "RustSbt721",
                    "RevealToken721",
                ])
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
    let content_hash: String = matches
        .value_of(ARGS_CONTENT_HASH)
        .unwrap_or_default()
        .to_string();
    let package: String = matches
        .value_of(ARGS_PACKAGE)
        .unwrap_or("EthersRs")
        .to_string();
    let package = Package::from_str(&package).ok().unwrap();
    let network: String = matches
        .value_of(ARGS_NETWORK)
        .unwrap_or("Ethereum")
        .to_string();
    let network = Network::from_str(&network).ok().unwrap();
    let contract: String = matches
        .value_of(ARGS_CONTRACT)
        .unwrap_or("RustToken721")
        .to_string();
    let contract = Contract::from_str(&contract).ok().unwrap();
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

    let result: CliResult<()> = match matches.value_of(COMMAND).unwrap() {
        COMMAND_BALANCE => match package {
            Package::EthersRs => impl_ethers_rs::get_balance(network)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::get_balance(network)
                .await
                .map_err(Error::from),
        },
        COMMAND_SEND_ETH => match package {
            Package::EthersRs => impl_ethers_rs::send_eth(network, ether, to_address)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::send_eth(network, ether, to_address)
                .await
                .map_err(Error::from),
        },
        COMMAND_CREATE_METADATA => {
            if !image_url.is_empty() {
                ipfs::create_metadata_from_url(name, description, image_url)
                    .await
                    .map_err(Error::from)
            } else {
                ipfs::create_metadata_from_file(name, description, image_filename)
                    .await
                    .map_err(Error::from)
            }
        }
        COMMAND_MINT => match package {
            Package::EthersRs => impl_ethers_rs::mint(contract, network, content_hash, amount)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::mint(contract, network, content_hash, amount)
                .await
                .map_err(Error::from),
        },
        COMMAND_TOKEN_INFO => match package {
            Package::EthersRs => impl_ethers_rs::show_token_info(contract, network)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::show_token_info(contract, network)
                .await
                .map_err(Error::from),
        },
        COMMAND_KEY_GEN => impl_ethers_rs::generate_keys().await.map_err(Error::from),
        COMMAND_SIGN => impl_ethers_rs::sign(message).await.map_err(Error::from),
        COMMAND_VERIFY => impl_ethers_rs::verify(signature, message)
            .await
            .map_err(Error::from),
        COMMAND_DEPLOY_TOKEN => match package {
            Package::EthersRs => impl_ethers_rs::deploy(contract, network)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::deploy(contract, network)
                .await
                .map_err(Error::from),
        },
        COMMAND_UPDATE_TIME => impl_ethers_rs::update_time(network)
            .await
            .map_err(Error::from),
        _ => Err(Error::Internal("unknown command".to_string())),
    };

    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }
}

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Package {
    EthersRs,
    RustWeb3,
}

pub type CliResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialOrd, PartialEq, Clone)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<impl_ethers_rs::Error> for Error {
    fn from(e: impl_ethers_rs::Error) -> Self {
        let msg = format!("ethers-rs error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<impl_rust_web3::Error> for Error {
    fn from(e: impl_rust_web3::Error) -> Self {
        let msg = format!("rust-web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ipfs::Error> for Error {
    fn from(e: ipfs::Error) -> Self {
        let msg = format!("ipfs error: {:?}", e);
        Self::Internal(msg)
    }
}
