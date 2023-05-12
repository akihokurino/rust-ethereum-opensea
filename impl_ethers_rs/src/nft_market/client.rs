use crate::{ether_to_wei, query_contract, transaction_contract, wei_to_ether, EthersResult};
use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::types::U256;
use prelude::*;
use std::env;

#[derive(Clone, Debug)]
pub struct Client {
    pub wallet_secret: String,
    pub address: Address,
    pub abi: Abi,
    pub network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        let wallet_secret =
            env::var("NFT_MARKET_OWNER_SECRET").expect("NFT_MARKET_OWNER_SECRET must be set");

        Client {
            wallet_secret,
            address: network.nft_market_address().parse::<Address>().unwrap(),
            abi: serde_json::from_str(include_str!("abi.json").trim()).unwrap(),
            network,
        }
    }

    pub async fn get_sell_order_keys(&self) -> EthersResult<Vec<String>> {
        let res: Vec<String> = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, Vec<String>>("getSellOrderKeys", ())?
        .call()
        .await?;

        Ok(res)
    }

    pub async fn get_all_sell_order(&self) -> EthersResult<Vec<NFT>> {
        let res: Vec<(Address, U256, Address, U256, String)> = query_contract(
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, Vec<(Address, U256, Address, U256, String)>>("getAllSellOrders", ())?
        .call()
        .await?;

        let items: Vec<NFT> = res
            .into_iter()
            .map(
                |(contract_address, token_id, seller, price, token_uri)| NFT {
                    contract_address,
                    token_id,
                    seller,
                    price: wei_to_ether(price),
                    token_uri,
                },
            )
            .collect();

        Ok(items)
    }

    pub async fn sell_order(
        &self,
        seller_wallet_secret: String,
        contract_address: String,
        token_id: u128,
        ether: f64,
    ) -> EthersResult<()> {
        let call = transaction_contract(
            seller_wallet_secret,
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>(
            "sellOrder",
            (
                contract_address.parse::<Address>().unwrap(),
                token_id,
                ether_to_wei(ether),
            ),
        )?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn cancel_order(
        &self,
        seller_wallet_secret: String,
        contract_address: String,
        token_id: u128,
    ) -> EthersResult<()> {
        let call = transaction_contract(
            seller_wallet_secret,
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>(
            "cancelOrder",
            (contract_address.parse::<Address>().unwrap(), token_id),
        )?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn cancel_order_by_admin(
        &self,
        contract_address: String,
        token_id: u128,
    ) -> EthersResult<()> {
        let call = transaction_contract(
            self.wallet_secret.to_owned(),
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>(
            "cancelOrderByAdmin",
            (contract_address.parse::<Address>().unwrap(), token_id),
        )?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE);
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }

    pub async fn buy_order(
        &self,
        buyer_wallet_secret: String,
        contract_address: String,
        token_id: u128,
        ether: f64,
    ) -> EthersResult<()> {
        let call = transaction_contract(
            buyer_wallet_secret,
            self.address.to_owned(),
            self.abi.to_owned(),
            self.network.to_owned(),
        )
        .await
        .method::<_, H256>(
            "buyOrder",
            (contract_address.parse::<Address>().unwrap(), token_id),
        )?
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE)
        .value(ether_to_wei(ether));
        let tx = call.send().await?;
        let receipt = tx.await?;

        println!("{:?}", receipt);

        Ok(())
    }
}

#[derive(Debug)]
pub struct NFT {
    pub contract_address: Address,
    pub token_id: U256,
    pub seller: Address,
    pub price: f64,
    pub token_uri: String,
}
