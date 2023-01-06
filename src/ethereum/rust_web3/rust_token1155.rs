use crate::error::CliResult;
use crate::ethereum::rust_web3::parse_address;
use crate::ethereum::{GAS_LIMIT, GAS_PRICE};
use crate::model::Network;
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
    #[allow(dead_code)]
    wallet_address: String,
    wallet_secret: String,
    contract_address: String,
    network: Network,
}

impl Client {
    #[allow(unused)]
    pub fn new(network: Network) -> Self {
        let transport = Http::new(&network.chain_url()).ok().unwrap();
        let cli = Web3::new(transport);

        let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            cli,
            wallet_address,
            wallet_secret,
            contract_address: network.rust_token1155_address(),
            network,
        }
    }

    fn contract(&self) -> CliResult<Contract<Http>> {
        let contract = Contract::from_json(
            self.cli.eth(),
            parse_address(self.contract_address.to_owned()).unwrap(),
            include_bytes!("rust-token1155.abi.json"),
        )?;
        Ok(contract)
    }

    #[allow(unused)]
    pub async fn simple_query<T: Tokenizable + std::fmt::Debug>(
        &self,
        method: &str,
    ) -> CliResult<T> {
        let c = self.contract()?;
        let result = c.query(method, (), None, Options::default(), None);
        let result: T = result.await?;

        Ok(result)
    }

    #[allow(unused)]
    pub async fn mint(&self, hash: String, amount: u128) -> CliResult<()> {
        let prev_key = SecretKey::from_str(&self.wallet_secret).unwrap();

        let c = self.contract()?;
        let result = c
            .signed_call_with_confirmations(
                "mint",
                (
                    parse_address(self.wallet_address.to_owned()).unwrap(),
                    hash,
                    amount,
                ),
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

    #[allow(unused)]
    pub async fn deploy(&self) -> CliResult<()> {
        let prev_key = SecretKey::from_str(&self.wallet_secret).unwrap();

        let contract = Contract::deploy(self.cli.eth(), include_bytes!("rust-token1155.abi.json"))?
            .confirmations(1)
            .poll_interval(time::Duration::from_secs(10))
            .options(Options::with(|opt| {
                opt.gas = Some(U256::from(GAS_LIMIT));
                opt.gas_price = Some(U256::from(GAS_PRICE));
            }))
            .sign_with_key_and_execute(
                include_str!("rust-token1155.bin").trim(),
                (),
                SecretKeyRef::from(&prev_key),
                Some(self.network.chain_id()),
            )
            .await?;

        println!("deployed erc1155 to: {:?}", contract.address());

        Ok(())
    }
}
