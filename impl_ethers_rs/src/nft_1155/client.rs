use crate::{deploy_contract, query_contract, transaction_contract, EthersResult};
use ethers::abi::Abi;
use ethers::prelude::*;
use prelude::*;
use std::env;
use std::str::FromStr;

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
            address: network.nft_1155_address().parse::<Address>().unwrap(),
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

    pub async fn latest_token_id(&self) -> EthersResult<u128> {
        let res = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, u128>("latestTokenId", ())?
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

    pub async fn total_owned(&self) -> EthersResult<u128> {
        let res = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, u128>("totalOwned", ())?
        .call()
        .await?;
        Ok(res)
    }

    pub async fn mint(&self, hash: String, amount: u128) -> EthersResult<()> {
        let call = transaction_contract(
            self.wallet_secret.to_owned(),
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>("mint", (hash, amount))?
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
        .method::<_, H256>(
            "safeTransferFrom",
            (
                self.wallet_address,
                to,
                token_id,
                1 as u64,
                Bytes::from_str("").unwrap(),
            ),
        )?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn set_approval_for_all(&self) -> EthersResult<()> {
        let call = transaction_contract(
            self.wallet_secret.to_owned(),
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>(
            "setApprovalForAll",
            (
                self.network
                    .nft_market_address()
                    .parse::<Address>()
                    .unwrap(),
                true,
            ),
        )?
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
