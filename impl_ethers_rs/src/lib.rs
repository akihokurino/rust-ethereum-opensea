use common::*;
use ethers::core::k256::elliptic_curve::sec1::ToEncodedPoint;
use ethers::prelude::*;
use ethers_signers::{LocalWallet, Signer, Wallet, WalletError};
use std::env;
use std::str::FromStr;
use thiserror::Error as ThisErr;

pub mod reveal_token_721;
pub mod rust_sbt_721;
pub mod rust_token_1155;
pub mod rust_token_721;

pub async fn get_balance(network: Network) -> EthersResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let wallet = wallet_secret
        .parse::<LocalWallet>()?
        .with_chain_id(network.chain_id());

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet.to_owned())
        .await
        .unwrap();

    let balance = client.get_balance(wallet.address(), None).await.unwrap();

    println!(
        "balance: {:?}",
        common::unit::to_ether(balance.to_string().as_str(), "wei")
    );

    Ok(())
}

pub async fn send_eth(network: Network, eth: f64, to: String) -> EthersResult<()> {
    let to = to.to_owned().parse::<Address>().unwrap();
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let wallet = wallet_secret
        .parse::<LocalWallet>()?
        .with_chain_id(network.chain_id());

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet)
        .await
        .unwrap();

    let wei = common::unit::to_wei(eth.to_string().as_str(), "ether");
    let wei: u128 = wei.parse().unwrap();
    let wei = U256::from(wei);

    let tx = TransactionRequest::new()
        .to(to)
        .value(wei)
        .gas(GAS_LIMIT)
        .gas_price(GAS_PRICE)
        .chain_id(network.chain_id());

    let res = client.send_transaction(tx, None).await.unwrap();
    let receipt = res.confirmations(1).await.unwrap();

    println!("sendEth: {:?}", receipt);

    Ok(())
}

pub async fn mint(
    target: Contract,
    network: Network,
    hash: String,
    amount: u128,
) -> EthersResult<()> {
    match target {
        Contract::RustToken721 => {
            let cli = rust_token_721::client::Client::new(network);
            cli.mint(hash.clone()).await
        }
        Contract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            cli.mint(hash.clone(), amount).await
        }
        Contract::RustSbt721 => {
            let cli = rust_sbt_721::client::Client::new(network);
            cli.mint(hash.clone()).await
        }
        Contract::RevealToken721 => {
            let cli = reveal_token_721::client::Client::new(network);
            cli.mint(hash.clone()).await
        }
    }?;
    Ok(())
}

pub async fn deploy(target: Contract, network: Network) -> EthersResult<()> {
    match target {
        Contract::RustToken721 => {
            let cli = rust_token_721::client::Client::new(network);
            cli.deploy().await
        }
        Contract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            cli.deploy().await
        }
        Contract::RustSbt721 => {
            let cli = rust_sbt_721::client::Client::new(network);
            cli.deploy().await
        }
        Contract::RevealToken721 => {
            let cli = reveal_token_721::client::Client::new(network);
            cli.deploy().await
        }
    }?;
    Ok(())
}

pub async fn show_token_info(target: Contract, network: Network) -> EthersResult<()> {
    match target {
        Contract::RustToken721 => {
            let cli = rust_token_721::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RustToken721 info: {}", network.rust_token_721_address());
            println!("name = {}", cli.simple_query::<String>("name").await?);
            println!(
                "latestTokenId = {}",
                cli.simple_query::<u128>("latestTokenId").await?
            );
            println!(
                "totalSupply = {:?}",
                cli.simple_query::<u128>("totalSupply").await?
            );
            println!(
                "totalOwned = {:?}",
                cli.simple_query::<u128>("totalOwned").await?
            );
            println!("------------------------------------------------------------");
        }
        Contract::RustToken1155 => {
            let cli = rust_token_1155::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RustToken1155 info: {}", network.rust_token_1155_address());
            println!("name = {}", cli.simple_query::<String>("name").await?);
            println!(
                "latestTokenId = {}",
                cli.simple_query::<u128>("latestTokenId").await?
            );
            println!(
                "totalSupply = {:?}",
                cli.simple_query::<u128>("totalSupply").await?
            );
            println!(
                "totalOwned = {:?}",
                cli.simple_query::<u128>("totalOwned").await?
            );
            println!("------------------------------------------------------------");
        }
        Contract::RustSbt721 => {
            let cli = rust_sbt_721::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RustToken1155 info: {}", network.rust_sbt_721_address());
            println!("name = {}", cli.simple_query::<String>("name").await?);
            println!(
                "latestTokenId = {}",
                cli.simple_query::<u128>("latestTokenId").await?
            );
            println!(
                "totalSupply = {:?}",
                cli.simple_query::<u128>("totalSupply").await?
            );
            println!(
                "totalOwned = {:?}",
                cli.simple_query::<u128>("totalOwned").await?
            );
            println!("------------------------------------------------------------");
        }
        Contract::RevealToken721 => {
            let cli = reveal_token_721::client::Client::new(network);
            println!("------------------------------------------------------------");
            println!("RevealToken info: {}", network.reveal_token_721_address());
            println!("name = {}", cli.simple_query::<String>("name").await?);
            println!(
                "totalSupply = {:?}",
                cli.simple_query::<u128>("totalSupply").await?
            );
            println!(
                "getCurrentHour = {}",
                cli.simple_query::<u128>("getCurrentHour").await?
            );
            println!("------------------------------------------------------------");
        }
    }

    Ok(())
}

pub async fn update_time(network: Network) -> EthersResult<()> {
    if network == Network::Ethereum {
        let cli = reveal_token_721::client::Client::new(network);
        cli.update_time().await?;
    }

    Ok(())
}

#[allow(unused)]
pub async fn generate_keys() -> EthersResult<()> {
    let seckey =
        ethers::core::k256::elliptic_curve::SecretKey::<ethers::core::k256::Secp256k1>::random(
            &mut rand::thread_rng(),
        );
    let seckey_str = ethers::utils::hex::encode(seckey.to_be_bytes().as_slice());
    let pubkey = seckey.public_key();
    let pubkey_encoded = pubkey.to_encoded_point(false);
    let pubkey_str = ethers::utils::hex::encode(pubkey_encoded.as_bytes());
    let address = ethers::core::types::Address::from_slice(
        &ethers::utils::keccak256(&pubkey_encoded.as_bytes()[1..])[12..],
    );
    let address_str = format!("{:?}", address);

    println!("secret: {}", seckey_str);
    println!("pubkey: {}", pubkey_str);
    println!("address {}", address_str);

    Ok(())
}

pub async fn sign(message: String) -> EthersResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let wallet = wallet_secret.parse::<LocalWallet>()?;

    let signature = wallet.sign_message(message).await?;

    println!("result: {}", signature.to_string());

    Ok(())
}

pub async fn verify(signature: String, message: String) -> EthersResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let wallet = wallet_secret.parse::<LocalWallet>()?;

    let sig = Signature::from_str(&signature).unwrap();

    match sig.verify(message, wallet.address()) {
        Ok(_) => println!("verified by {:?}!", wallet.address()),
        Err(_) => println!("cannot verified by {:?}!", wallet.address()),
    }

    Ok(())
}

pub type EthersResult<T> = Result<T, Error>;

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<AbiError> for Error {
    fn from(e: AbiError) -> Self {
        let msg = format!("ethers contract abi error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ContractError<Provider<Http>>> for Error {
    fn from(e: ContractError<Provider<Http>>) -> Self {
        let msg = format!("ethers contract call error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<WalletError> for Error {
    fn from(e: WalletError) -> Self {
        let msg = format!("ethers contract wallet error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ContractError<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
    for Error
{
    fn from(
        e: ContractError<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    ) -> Self {
        match e {
            ContractError::DecodingError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ContractError::AbiError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ContractError::DetokenizationError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ContractError::MiddlewareError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ContractError::ProviderError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ContractError::ConstructorError => {
                let msg =
                    format!("ethers contract sign error: constructor is not defined in the ABI");
                Self::Internal(msg)
            }
            ContractError::ContractNotDeployed => {
                let msg = format!("ethers contract sign error: Contract was not deployed");
                Self::Internal(msg)
            }
        }
    }
}

impl From<ProviderError> for Error {
    fn from(e: ProviderError) -> Self {
        let msg = format!("ethers transaction error: {:?}", e);
        Self::Internal(msg)
    }
}
