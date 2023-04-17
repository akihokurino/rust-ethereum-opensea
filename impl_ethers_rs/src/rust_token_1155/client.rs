use crate::EthersResult;
use prelude::*;
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
    wallet_secret: String,
    provider: Provider<Http>,
    address: Address,
    abi: Abi,
    network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            wallet_secret,
            provider: Provider::<Http>::try_from(network.chain_url()).unwrap(),
            address: network
                .rust_token_1155_address()
                .parse::<Address>()
                .unwrap(),
            abi: serde_json::from_str(include_str!("abi.json").trim()).unwrap(),
            network,
        }
    }

    pub async fn simple_query<T: Tokenizable + std::fmt::Debug>(
        &self,
        method: &str,
    ) -> EthersResult<T> {
        let contract = Contract::new(self.address, self.abi.to_owned(), self.provider.to_owned());
        let res = contract.method::<_, T>(method, ())?.call().await?;
        Ok(res)
    }

    pub async fn mint(&self, hash: String, amount: u128) -> EthersResult<()> {
        let wallet = self
            .wallet_secret
            .parse::<LocalWallet>()?
            .with_chain_id(self.network.chain_id());

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
            .method::<_, H256>("mint", (hash, amount))?
            .gas(GAS_LIMIT)
            .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("mint result: {:?}", receipt);

        Ok(())
    }

    pub async fn deploy(&self) -> EthersResult<()> {
        let wallet = self
            .wallet_secret
            .parse::<LocalWallet>()?
            .with_chain_id(self.network.chain_id());

        let client = SignerMiddleware::new_with_provider_chain(self.provider.to_owned(), wallet)
            .await
            .unwrap();
        let client = Arc::new(client);

        let bytecode = include_str!("bin").trim();
        let factory = ContractFactory::new(
            self.abi.to_owned(),
            Bytes::from_str(bytecode).unwrap(),
            client.clone(),
        );

        let mut deployer = factory.deploy(())?;
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

        println!("deployed to: {:?}", contract.address());

        Ok(())
    }
}
