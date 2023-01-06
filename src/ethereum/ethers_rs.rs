use crate::error::CliResult;
use crate::ethereum::{to_ether, to_wei, GAS_LIMIT, GAS_PRICE};
use crate::model::Network;
use bigdecimal::BigDecimal;
use ethers::prelude::*;
use std::env;

pub mod reveal_token721;
pub mod rust_token1155;
pub mod rust_token721;

pub async fn get_balance(network: Network) -> CliResult<BigDecimal> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let wallet = wallet_secret
        .parse::<LocalWallet>()?
        .with_chain_id(network.chain_id());

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet.to_owned())
        .await
        .unwrap();

    let balance = client.get_balance(wallet.address(), None).await.unwrap();

    Ok(to_ether(balance.to_string().as_str(), "wei"))
}

pub async fn send_eth(network: Network, eth: f64, to: Address) -> CliResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let wallet = wallet_secret
        .parse::<LocalWallet>()?
        .with_chain_id(network.chain_id());

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet)
        .await
        .unwrap();

    let wei = to_wei(eth.to_string().as_str(), "ether");
    let wei: u128 = wei.parse().unwrap();
    let wei = U256::from(wei);

    let tx = TransactionRequest::new()
        .to(to)
        .value(wei)
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE)
        .chain_id(network.chain_id());

    let res = client.send_transaction(tx, None).await.unwrap();
    let receipt = res.confirmations(1).await.unwrap();

    println!("sendEth: {:?}", receipt);

    Ok(())
}
