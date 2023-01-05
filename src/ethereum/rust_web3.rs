pub mod rust_token1155;
pub mod rust_token721;

use crate::error::CliResult;
use crate::ethereum::{to_ether, to_wei, GAS_LIMIT, GAS_PRICE};
use crate::model::Network;
use bigdecimal::BigDecimal;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::signing::SecretKeyRef;
use web3::types::{Address, TransactionParameters, U256};
use web3::{transports, Web3};

pub fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}

#[allow(unused)]
pub async fn get_balance(network: Network) -> CliResult<BigDecimal> {
    let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");

    let transport = transports::Http::new(&network.chain_url())
        .ok()
        .expect("should set ethereum url");
    let cli = Web3::new(transport);

    let balance = cli
        .eth()
        .balance(parse_address(wallet_address).unwrap(), None)
        .await?;

    Ok(to_ether(balance.to_string().as_str(), "wei"))
}

#[allow(unused)]
pub async fn send_eth(network: Network, eth: f64, to: Address) -> CliResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let prev_key = SecretKey::from_str(&wallet_secret).unwrap();

    let transport = transports::Http::new(&network.chain_url())
        .ok()
        .expect("should set ethereum url");
    let cli = Web3::new(transport);

    let wei = to_wei(eth.to_string().as_str(), "ether");
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
