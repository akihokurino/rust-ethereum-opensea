use crate::{contract, deploy_contract, parse_address, Web3Result};
use prelude::*;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::Options;
use web3::signing::SecretKeyRef;
use web3::types::{Address, U256};

#[derive(Clone, Debug)]
pub struct Client {
    wallet_secret: String,
    contract_address: Address,
    network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            wallet_secret,
            contract_address: parse_address(network.rust_token_721_address().to_owned()).unwrap(),
            network,
        }
    }

    pub async fn name(&self) -> Web3Result<String> {
        let contract = contract(
            self.contract_address.to_owned(),
            include_bytes!("abi.json"),
            self.network.to_owned(),
        );
        let result = contract.query("name", (), None, Options::default(), None);
        let result: String = result.await?;

        Ok(result)
    }

    pub async fn latest_token_id(&self) -> Web3Result<u128> {
        let contract = contract(
            self.contract_address.to_owned(),
            include_bytes!("abi.json"),
            self.network.to_owned(),
        );
        let result = contract.query("latestTokenId", (), None, Options::default(), None);
        let result: u128 = result.await?;

        Ok(result)
    }

    pub async fn total_supply(&self) -> Web3Result<u128> {
        let contract = contract(
            self.contract_address.to_owned(),
            include_bytes!("abi.json"),
            self.network.to_owned(),
        );
        let result = contract.query("totalSupply", (), None, Options::default(), None);
        let result: u128 = result.await?;

        Ok(result)
    }

    pub async fn total_owned(&self) -> Web3Result<u128> {
        let contract = contract(
            self.contract_address.to_owned(),
            include_bytes!("abi.json"),
            self.network.to_owned(),
        );
        let result = contract.query("totalOwned", (), None, Options::default(), None);
        let result: u128 = result.await?;

        Ok(result)
    }

    pub async fn mint(&self, hash: String) -> Web3Result<()> {
        let secret_key = SecretKey::from_str(&self.wallet_secret).unwrap();
        let contract = contract(
            self.contract_address.to_owned(),
            include_bytes!("abi.json"),
            self.network.to_owned(),
        );
        let result = contract
            .signed_call_with_confirmations(
                "mint",
                hash,
                Options::with(|opt| {
                    opt.gas = Some(U256::from(GAS_LIMIT));
                    opt.gas_price = Some(U256::from(GAS_PRICE));
                }),
                1,
                SecretKeyRef::from(&secret_key),
            )
            .await?;

        println!("tx id: {:?}", result.transaction_hash);
        println!("gas used: {:?}", result.gas_used.unwrap_or_default());
        println!("status: {:?}", result.status.unwrap_or_default());

        Ok(())
    }

    pub async fn deploy(&self) -> Web3Result<()> {
        let contract = deploy_contract(
            self.wallet_secret.to_owned(),
            include_bytes!("abi.json"),
            self.network.to_owned(),
            include_str!("bin").trim(),
        )
        .await?;

        println!("deployed to: {:?}", contract.address());

        Ok(())
    }
}
