use crate::{parse_address, Web3Result};
use common::*;
use secp256k1::SecretKey;
use std::str::FromStr;
use std::{env, time};
use web3::contract::tokens::Tokenizable;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::U256;
use web3::Web3;

#[derive(Clone, Debug)]
pub struct Client {
    cli: Web3<Http>,
    wallet_secret: String,
    contract_address: String,
    network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        let transport = Http::new(&network.chain_url()).ok().unwrap();
        let cli = Web3::new(transport);

        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            cli,
            wallet_secret,
            contract_address: network.rust_token_721_address(),
            network,
        }
    }

    fn contract(&self) -> Web3Result<Contract<Http>> {
        let contract = Contract::from_json(
            self.cli.eth(),
            parse_address(self.contract_address.to_owned()).unwrap(),
            include_bytes!("abi.json"),
        )?;
        Ok(contract)
    }

    pub async fn simple_query<T: Tokenizable + std::fmt::Debug>(
        &self,
        method: &str,
    ) -> Web3Result<T> {
        let c = self.contract()?;
        let result = c.query(method, (), None, Options::default(), None);
        let result: T = result.await?;

        Ok(result)
    }

    pub async fn mint(&self, hash: String) -> Web3Result<()> {
        let prev_key = SecretKey::from_str(&self.wallet_secret).unwrap();

        let c = self.contract()?;
        let result = c
            .signed_call_with_confirmations(
                "mint",
                hash,
                Options::with(|opt| {
                    opt.gas = Some(U256::from(GAS_LIMIT));
                    opt.gas_price = Some(U256::from(GAS_PRICE));
                }),
                1,
                SecretKeyRef::from(&prev_key),
            )
            .await?;

        println!("tx id: {:?}", result.transaction_hash);
        println!("gas used: {:?}", result.gas_used.unwrap_or_default());
        println!("status: {:?}", result.status.unwrap_or_default());

        Ok(())
    }

    pub async fn deploy(&self) -> Web3Result<()> {
        let prev_key = SecretKey::from_str(&self.wallet_secret).unwrap();

        let contract = Contract::deploy(self.cli.eth(), include_bytes!("abi.json"))?
            .confirmations(1)
            .poll_interval(time::Duration::from_secs(10))
            .options(Options::with(|opt| {
                opt.gas = Some(U256::from(GAS_LIMIT));
                opt.gas_price = Some(U256::from(GAS_PRICE));
            }))
            .sign_with_key_and_execute(
                include_str!("bin").trim(),
                (),
                SecretKeyRef::from(&prev_key),
                Some(self.network.chain_id()),
            )
            .await?;

        println!("deployed to: {:?}", contract.address());

        Ok(())
    }
}
