use crate::{deploy_contract, query_contract, transaction_contract, EthersResult};
use ethers::abi::Abi;
use ethers::prelude::*;
use prelude::*;
use std::env;

#[derive(Clone, Debug)]
pub struct Client {
    wallet_address: Address,
    wallet_secret: String,
    address: Address,
    abi: Abi,
    network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            wallet_address: wallet_address.parse::<Address>().unwrap(),
            wallet_secret,
            address: network.reveal_nft_address().parse::<Address>().unwrap(),
            abi: serde_json::from_str(include_str!("abi.json").trim()).unwrap(),
            network,
        }
    }

    pub async fn name(&self) -> EthersResult<String> {
        let res = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, String>("name", ())?
        .call()
        .await?;
        Ok(res)
    }

    pub async fn total_supply(&self) -> EthersResult<u128> {
        let res = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, u128>("totalSupply", ())?
        .call()
        .await?;
        Ok(res)
    }

    pub async fn get_current_hour(&self) -> EthersResult<u128> {
        let res = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, u128>("getCurrentHour", ())?
        .call()
        .await?;
        Ok(res)
    }

    pub async fn mint(&self, hash: String) -> EthersResult<()> {
        let call = transaction_contract(
            self.wallet_secret.to_owned(),
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>("mint", hash)?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn update_time(&self) -> EthersResult<()> {
        let call = transaction_contract(
            self.wallet_secret.to_owned(),
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>("updateTime", ())?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn transfer(&self, to: Address, token_id: u128) -> EthersResult<()> {
        let call = transaction_contract(
            self.wallet_secret.to_owned(),
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>("safeTransferFrom", (self.wallet_address, to, token_id))?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn deploy(&self) -> EthersResult<()> {
        let contract = deploy_contract(
            self.wallet_secret.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
            include_str!("bin").trim(),
        )
        .await;

        println!("deployed to: {:?}", contract.address());

        Ok(())
    }
}
