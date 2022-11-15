use crate::error::CliResult;
use crate::ethereum::{GAS_LIMIT, GAS_PRICE};
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::prelude::*;
use ethers_signers::{LocalWallet, Signer};
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
            address: env::var("HELLO_ADDRESS")
                .expect("HELLO_ADDRESS must be set")
                .parse::<Address>()
                .unwrap(),
            abi: serde_json::from_str(include_str!("hello.abi.json").trim()).unwrap(),
        }
    }

    pub async fn simple_query<T: abi::Tokenizable + std::fmt::Debug>(
        &self,
        method: &str,
    ) -> CliResult<()> {
        let contract = Contract::new(
            self.address.clone(),
            self.abi.clone(),
            self.provider.clone(),
        );

        let res = contract.method::<_, T>(method, ())?.call().await?;

        println!("{}: {:?}", method, res);

        Ok(())
    }

    pub async fn set_message(&self, message: String) -> CliResult<()> {
        let wallet = self.wallet_secret.parse::<LocalWallet>()?;

        let client = SignerMiddleware::new(
            self.provider.clone(),
            wallet.with_chain_id(
                env::var("ETHEREUM_CHAIN_ID")
                    .expect("ETHEREUM_CHAIN_ID must be set")
                    .parse::<u64>()
                    .unwrap(),
            ),
        );

        let contract = Contract::new(self.address.clone(), self.abi.clone(), client);
        let func = contract
            .method::<_, H256>("setMessage", message)?
            .gas(GAS_LIMIT)
            .gas_price(GAS_PRICE);
        let tx = func.send().await?;
        let receipt = tx.await.unwrap();

        println!("setMessage: {:?}", receipt);

        Ok(())
    }
}
