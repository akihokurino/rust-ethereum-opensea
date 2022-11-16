use crate::error::CliResult;
use crate::ethereum::rust_web3::parse_address;
use crate::ethereum::{GAS_LIMIT, GAS_PRICE};
use secp256k1::SecretKey;
use std::str::FromStr;
use std::{env, time};
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
}

impl Client {
    pub fn new() -> Self {
        let chain_url = env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set");
        let transport = Http::new(&chain_url).ok().unwrap();
        let cli = Web3::new(transport);

        let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
        let contract_address = env::var("ERC721_ADDRESS").expect("ERC721_ADDRESS must be set");

        Client {
            cli,
            wallet_address,
            wallet_secret,
            contract_address,
        }
    }

    fn contract(&self) -> CliResult<Contract<Http>> {
        let contract = Contract::from_json(
            self.cli.eth(),
            parse_address(self.contract_address.clone()).unwrap(),
            include_bytes!("rust-token721.abi.json"),
        )?;
        Ok(contract)
    }

    pub async fn get_name(&self) -> CliResult<String> {
        let c = self.contract()?;
        let result = c.query("name", (), None, Options::default(), None);
        let name: String = result.await?;

        Ok(name)
    }

    pub async fn get_already_used_names(&self) -> CliResult<Vec<String>> {
        let c = self.contract()?;
        let result = c.query("usedTokenNames", (), None, Options::default(), None);
        let names: Vec<String> = result.await?;

        Ok(names)
    }

    pub async fn get_current_supply(&self) -> CliResult<u128> {
        let c = self.contract()?;
        let result = c.query("currentSupply", (), None, Options::default(), None);
        let supply: u128 = result.await?;

        Ok(supply)
    }

    pub async fn mint(&self, hash: String) -> CliResult<()> {
        let prev_key = SecretKey::from_str(&self.wallet_secret).unwrap();

        let c = self.contract()?;
        let result = c
            .signed_call_with_confirmations(
                "mint",
                (parse_address(self.wallet_address.clone()).unwrap(), hash),
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

    pub async fn deploy(&self) -> CliResult<()> {
        let prev_key = SecretKey::from_str(&self.wallet_secret).unwrap();
        let chain_id = env::var("ETHEREUM_CHAIN_ID").expect("ETHEREUM_CHAIN_ID must be set");

        let contract = Contract::deploy(self.cli.eth(), include_bytes!("rust-token721.abi.json"))?
            .confirmations(1)
            .poll_interval(time::Duration::from_secs(10))
            .options(Options::with(|opt| {
                opt.gas = Some(U256::from(GAS_LIMIT));
                opt.gas_price = Some(U256::from(GAS_PRICE));
            }))
            .sign_with_key_and_execute(
                include_str!("rust-token721.bin").trim(),
                (),
                SecretKeyRef::from(&prev_key),
                Some(chain_id.parse().unwrap()),
            )
            .await?;

        println!("deployed erc721 to: {:?}", contract.address());

        Ok(())
    }
}
