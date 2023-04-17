use prelude::*;
use secp256k1::SecretKey;
use std::str::FromStr;
use std::{env, time};
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::{Address, TransactionParameters, U256};
use web3::Web3;

pub mod rust_token_1155;
pub mod rust_token_721;

fn contract(contract_address: Address, abi: &[u8], network: Network) -> Contract<Http> {
    let transport = Http::new(&network.chain_url()).ok().unwrap();
    Contract::from_json(Web3::new(transport).eth(), contract_address, abi).unwrap()
}

async fn deploy_contract(
    wallet_secret: String,
    abi: &[u8],
    network: Network,
    bytecode: &str,
) -> Web3Result<Contract<Http>> {
    let secret_key = SecretKey::from_str(&wallet_secret).unwrap();
    let transport = Http::new(&network.chain_url()).ok().unwrap();
    let contract = Contract::deploy(Web3::new(transport).eth(), abi)?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| {
            opt.gas = Some(U256::from(GAS_LIMIT));
            opt.gas_price = Some(U256::from(GAS_PRICE));
        }))
        .sign_with_key_and_execute(
            bytecode,
            (),
            SecretKeyRef::from(&secret_key),
            Some(network.chain_id()),
        )
        .await?;

    Ok(contract)
}

fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}

pub async fn get_balance(network: Network) -> Web3Result<()> {
    let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");

    let transport = Http::new(&network.chain_url())
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

    let transport = Http::new(&network.chain_url())
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
            println!("name = {}", cli.name().await?);
            println!("latestTokenId = {}", cli.latest_token_id().await?);
            println!("totalSupply = {:?}", cli.total_supply().await?);
            println!("totalOwned = {:?}", cli.total_owned().await?);
            println!("------------------------------------------------------------");
        }
        TargetContract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RustToken1155 info: {}", network.rust_token_1155_address());
            println!("name = {}", cli.name().await?);
            println!("latestTokenId = {}", cli.latest_token_id().await?);
            println!("totalSupply = {:?}", cli.total_supply().await?);
            println!("totalOwned = {:?}", cli.total_owned().await?);
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
