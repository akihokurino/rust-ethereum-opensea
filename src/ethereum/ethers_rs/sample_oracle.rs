use crate::error::CliResult;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::prelude::*;
use std::env;

#[derive(Clone, Debug)]
pub struct Client {
    wallet_address: String,
    wallet_secret: String,
    provider: Provider<Http>,
    address: Address,
    abi: Abi,
}

impl Client {
    pub fn new() -> Self {
        let chain_url = env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set");
        let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            wallet_address,
            wallet_secret,
            provider: Provider::<Http>::try_from(chain_url).unwrap(),
            address: env::var("SAMPLE_ORACLE_ADDRESS")
                .expect("SAMPLE_ORACLE_ADDRESS must be set")
                .parse::<Address>()
                .unwrap(),
            abi: serde_json::from_str(include_str!("sample-oracle.abi.json").trim()).unwrap(),
        }
    }

    pub async fn query<T: abi::Tokenizable + std::fmt::Debug>(&self, method: &str) -> CliResult<()> {
        let contract = Contract::new(
            self.address.clone(),
            self.abi.clone(),
            self.provider.clone(),
        );

        let res = contract.method::<_, T>(method, ())?.call().await?;

        println!("{}: {:?}", method, res);

        Ok(())
    }
}
