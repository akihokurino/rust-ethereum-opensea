use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::core::k256::elliptic_curve::sec1::ToEncodedPoint;
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers_signers::{LocalWallet, Signer, Wallet, WalletError};
use prelude::*;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

pub mod event;
pub mod nft_1155;
pub mod nft_721;
pub mod nft_market;
pub mod reveal_nft_721;
pub mod sbt_721;

fn wei_to_ether(wei_amount: U256) -> f64 {
    let ether_float = wei_amount.to_string().parse::<f64>().unwrap() * (10.0f64).powi(-18);
    ether_float
}

fn ether_to_wei(ether_amount: f64) -> U256 {
    let wei_float = ether_amount * (10.0f64).powi(18);
    U256::from(wei_float.round() as u64)
}

fn query_contract(
    contract_address: Address,
    abi: Abi,
    network: Network,
) -> Contract<Provider<Http>> {
    let provider = Arc::new(Provider::<Http>::try_from(network.chain_url()).unwrap());
    Contract::new(contract_address, abi, provider)
}

async fn transaction_contract(
    wallet_secret: String,
    contract_address: Address,
    abi: Abi,
    network: Network,
) -> Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
    let wallet = wallet_secret
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(network.chain_id());

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let client = SignerMiddleware::new_with_provider_chain(provider, wallet)
        .await
        .unwrap();
    let client = Arc::new(client);

    Contract::<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>::new(
        contract_address,
        abi,
        client.clone(),
    )
}

async fn deploy_contract(
    wallet_secret: String,
    abi: Abi,
    network: Network,
    bytecode: &str,
) -> Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>> {
    let wallet = wallet_secret
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(network.chain_id());

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let client = SignerMiddleware::new_with_provider_chain(provider, wallet)
        .await
        .unwrap();
    let client = Arc::new(client);

    let factory = ContractFactory::new(abi, Bytes::from_str(bytecode).unwrap(), client.clone());

    let mut deployer = factory.deploy(()).unwrap();
    deployer.tx = TypedTransaction::Legacy(TransactionRequest {
        to: None,
        data: deployer.tx.data().cloned(),
        gas: Some(U256::from(GAS_LIMIT)),
        gas_price: Some(U256::from(GAS_PRICE)),
        ..Default::default()
    });
    deployer
        .confirmations(1 as usize)
        .legacy()
        .send()
        .await
        .unwrap()
}

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
        unit::to_ether(balance.to_string().as_str(), "wei")
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

    let wei = unit::to_wei(eth.to_string().as_str(), "ether");
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

pub fn to_address(from: String) -> Address {
    from.parse::<Address>().unwrap()
}

pub async fn generate_keys() -> EthersResult<()> {
    let seckey =
        k256::elliptic_curve::SecretKey::<k256::Secp256k1>::random(&mut rand::thread_rng());
    let seckey_str = ethers::utils::hex::encode(seckey.to_bytes().as_slice());
    let pubkey = seckey.public_key();
    let pubkey_encoded = pubkey.to_encoded_point(false);
    let pubkey_str = ethers::utils::hex::encode(pubkey_encoded.as_bytes());
    let address =
        Address::from_slice(&ethers::utils::keccak256(&pubkey_encoded.as_bytes()[1..])[12..]);
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

    println!("signature: {}", signature.to_string());

    Ok(())
}

pub async fn verify(signature: String, message: String) -> EthersResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let wallet = wallet_secret.parse::<LocalWallet>()?;

    let sig = Signature::from_str(&signature).unwrap();

    match sig.verify(message, wallet.address()) {
        Ok(_) => println!("verified by {:?}", wallet.address()),
        Err(_) => println!("cannot verified by {:?}", wallet.address()),
    }

    Ok(())
}

pub type EthersResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialOrd, PartialEq, Clone)]
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
            ContractError::ConstructorError => {
                let msg =
                    format!("ethers contract sign error: constructor is not defined in the ABI");
                Self::Internal(msg)
            }
            ContractError::ContractNotDeployed => {
                let msg = format!("ethers contract sign error: Contract was not deployed");
                Self::Internal(msg)
            }
            _ => {
                let msg = format!("ethers contract sign error");
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

impl From<abi::Error> for Error {
    fn from(e: abi::Error) -> Self {
        let msg = format!("ethers abi error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ContractError<Provider<Ws>>> for Error {
    fn from(e: ContractError<Provider<Ws>>) -> Self {
        let msg = format!("ethers ws error: {:?}", e);
        Self::Internal(msg)
    }
}
