use prelude::*;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::signing::SecretKeyRef;
use web3::types::{Address, TransactionParameters, U256};
use web3::{transports, Web3};

pub mod rust_token_1155;
pub mod rust_token_721;

fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}

pub async fn get_balance(network: Network) -> Web3Result<()> {
    let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");

    let transport = transports::Http::new(&network.chain_url())
        .ok()
        .expect("should set ethereum url");
    let cli = Web3::new(transport);

    let balance = cli
        .eth()
        .balance(parse_address(wallet_address).unwrap(), None)
        .await?;

    println!(
        "balance: {:?}",
        unit::to_ether(balance.to_string().as_str(), "wei")
    );

    Ok(())
}

pub async fn send_eth(network: Network, eth: f64, to: String) -> Web3Result<()> {
    let to = parse_address(to.to_owned()).unwrap();
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let prev_key = SecretKey::from_str(&wallet_secret).unwrap();

    let transport = transports::Http::new(&network.chain_url())
        .ok()
        .expect("should set ethereum url");
    let cli = Web3::new(transport);

    let wei = unit::to_wei(eth.to_string().as_str(), "ether");
    let wei: u128 = wei.parse().unwrap();
    let wei = U256::from(wei);
    println!("send wei: {}", &wei);

    let tx = TransactionParameters {
        to: Some(to),
        value: wei,
        gas: U256::from(GAS_LIMIT),
        gas_price: Some(U256::from(GAS_PRICE)),
        ..Default::default()
    };

    let signed = cli
        .accounts()
        .sign_transaction(tx, SecretKeyRef::from(&prev_key))
        .await?;
    let result = cli
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;

    println!("sendEth: {:?}", result);

    Ok(())
}

pub async fn mint(
    target: TargetContract,
    network: Network,
    hash: String,
    amount: u128,
) -> Web3Result<()> {
    match target {
        TargetContract::RustToken721 => {
            let cli = rust_token_721::client::Client::new(network);
            cli.mint(hash.clone()).await
        }
        TargetContract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            cli.mint(hash.clone(), amount).await
        }
        _ => return Err(Error::Internal("invalid params".to_string())),
    }?;
    Ok(())
}

pub async fn deploy(target: TargetContract, network: Network) -> Web3Result<()> {
    match target {
        TargetContract::RustToken721 => {
            let cli = rust_token_721::client::Client::new(network);
            cli.deploy().await
        }
        TargetContract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            cli.deploy().await
        }
        _ => return Err(Error::Internal("invalid params".to_string())),
    }?;
    Ok(())
}

pub async fn show_token_info(target: TargetContract, network: Network) -> Web3Result<()> {
    match target {
        TargetContract::RustToken721 => {
            let cli = rust_token_721::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RustToken721 info: {}", network.rust_token_721_address());
            println!("name = {}", cli.simple_query::<String>("name").await?);
            println!(
                "latestTokenId = {}",
                cli.simple_query::<u128>("latestTokenId").await?
            );
            println!(
                "totalSupply = {:?}",
                cli.simple_query::<u128>("totalSupply").await?
            );
            println!(
                "totalOwned = {:?}",
                cli.simple_query::<u128>("totalOwned").await?
            );
            println!("------------------------------------------------------------");
        }
        TargetContract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RustToken1155 info: {}", network.rust_token_1155_address());
            println!("name = {}", cli.simple_query::<String>("name").await?);
            println!(
                "latestTokenId = {}",
                cli.simple_query::<u128>("latestTokenId").await?
            );
            println!(
                "totalSupply = {:?}",
                cli.simple_query::<u128>("totalSupply").await?
            );
            println!(
                "totalOwned = {:?}",
                cli.simple_query::<u128>("totalOwned").await?
            );
            println!("------------------------------------------------------------");
        }
        _ => return Err(Error::Internal("invalid params".to_string())),
    }

    Ok(())
}

pub type Web3Result<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialOrd, PartialEq, Clone)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<web3::Error> for Error {
    fn from(e: web3::Error) -> Self {
        let msg = format!("rust_web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::ethabi::Error> for Error {
    fn from(e: web3::ethabi::Error) -> Self {
        let msg = format!("rust_web3 abi error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::contract::Error> for Error {
    fn from(e: web3::contract::Error) -> Self {
        let msg = format!("rust_web3 contract error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::contract::deploy::Error> for Error {
    fn from(e: web3::contract::deploy::Error) -> Self {
        let msg = format!("rust_web3 contract deploy error: {:?}", e);
        Self::Internal(msg)
    }
}
