use crate::error::CliResult;
use crate::ethereum::{to_ether, to_wei, GAS_LIMIT, GAS_PRICE};
use ethers::prelude::*;
use std::env;

pub mod hello;
pub mod sample_oracle;

pub async fn get_balance() -> CliResult<()> {
    let chain_url = env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set");
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(chain_url).unwrap();
    let wallet = wallet_secret.parse::<LocalWallet>()?.with_chain_id(
        env::var("ETHEREUM_CHAIN_ID")
            .expect("ETHEREUM_CHAIN_ID must be set")
            .parse::<u64>()
            .unwrap(),
    );

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet.to_owned())
        .await
        .unwrap();

    let balance = client.get_balance(wallet.address(), None).await.unwrap();

    println!("balance wei: {}", balance);
    println!(
        "balance ether: {}",
        to_ether(balance.to_string().as_str(), "wei")
    );

    Ok(())
}

pub async fn send_eth(eth: f64, to: Address) -> CliResult<()> {
    let chain_url = env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set");
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(chain_url).unwrap();
    let wallet = wallet_secret.parse::<LocalWallet>()?.with_chain_id(
        env::var("ETHEREUM_CHAIN_ID")
            .expect("ETHEREUM_CHAIN_ID must be set")
            .parse::<u64>()
            .unwrap(),
    );

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet)
        .await
        .unwrap();

    let wei = to_wei(eth.to_string().as_str(), "ether");
    let wei: u128 = wei.parse().unwrap();
    let wei = U256::from(wei);
    println!("send wei: {}", &wei);

    let tx = TransactionRequest::new()
        .to(to)
        .value(wei)
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE)
        .chain_id(
            env::var("ETHEREUM_CHAIN_ID")
                .expect("ETHEREUM_CHAIN_ID must be set")
                .parse::<u64>()
                .unwrap(),
        );

    let res = client.send_transaction(tx, None).await.unwrap();
    let receipt = res.confirmations(1).await.unwrap();

    println!("sendEth: {:?}", receipt);

    Ok(())
}
