use crate::error::CliResult;
use crate::ethereum::{GAS_LIMIT, GAS_PRICE};
use ethers::abi::{Abi, Tokenizable};
use ethers::contract::Contract;
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers_signers::{LocalWallet, Signer};
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Client {
    #[allow(dead_code)]
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

    pub async fn simple_query<T: Tokenizable + std::fmt::Debug>(
        &self,
        method: &str,
    ) -> CliResult<()> {
        let contract = Contract::new(self.address, self.abi.to_owned(), self.provider.to_owned());

        let res = contract.method::<_, T>(method, ())?.call().await?;

        println!("{}: {:?}", method, res);

        Ok(())
    }

    pub async fn set_message(&self, message: String) -> CliResult<()> {
        let wallet = self.wallet_secret.parse::<LocalWallet>()?.with_chain_id(
            env::var("ETHEREUM_CHAIN_ID")
                .expect("ETHEREUM_CHAIN_ID must be set")
                .parse::<u64>()
                .unwrap(),
        );

        let client = SignerMiddleware::new_with_provider_chain(self.provider.to_owned(), wallet)
            .await
            .unwrap();
        let client = Arc::new(client);

        let contract =
            Contract::<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>::new(
                self.address,
                self.abi.to_owned(),
                client.clone(),
            );

        let call = contract
            .method::<_, H256>("setMessage", message)?
            .gas(GAS_LIMIT)
            .gas_price(GAS_PRICE);
        let tx = call.send().await.unwrap();
        let receipt = tx.confirmations(1).await.unwrap();

        println!("setMessage: {:?}", receipt);

        Ok(())
    }

    pub async fn deploy(&self) -> CliResult<()> {
        let wallet = self.wallet_secret.parse::<LocalWallet>()?.with_chain_id(
            env::var("ETHEREUM_CHAIN_ID")
                .expect("ETHEREUM_CHAIN_ID must be set")
                .parse::<u64>()
                .unwrap(),
        );

        let client = SignerMiddleware::new_with_provider_chain(self.provider.to_owned(), wallet)
            .await
            .unwrap();
        let client = Arc::new(client);

        let bytecode = include_str!("hello.bin").trim();
        let factory = ContractFactory::new(
            self.abi.to_owned(),
            Bytes::from_str(bytecode).unwrap(),
            client.clone(),
        );

        let mut deployer = factory.deploy(()).unwrap();
        deployer.tx = TypedTransaction::Legacy(TransactionRequest {
            to: None,
            data: deployer.tx.data().cloned(),
            gas: Some(U256::from(GAS_LIMIT)),
            gas_price: Some(U256::from(GAS_PRICE)),
            ..Default::default()
        });
        let contract = deployer
            .confirmations(1 as usize)
            .legacy()
            .send()
            .await
            .unwrap();

        println!("deployed hello to: {:?}", contract.address());

        Ok(())
    }
}
