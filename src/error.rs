use aws_sdk_lambda::error::InvokeError;
use aws_sdk_lambda::types::SdkError;
use ethers::signers::{Wallet, WalletError};
use reqwest::StatusCode;
use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum CliError {
    #[error("invalid parameter error: {0}")]
    InvalidArgument(String),
    #[error("not found error")]
    NotFound,
    #[error("internal error: {0}")]
    Internal(String),
}

pub type CliResult<T> = Result<T, CliError>;

impl From<SdkError<InvokeError>> for CliError {
    fn from(e: SdkError<InvokeError>) -> Self {
        let msg = format!("lambda invoke error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        let msg = format!("json parse error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::Error> for CliError {
    fn from(e: web3::Error) -> Self {
        let msg = format!("rust_web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::ethabi::Error> for CliError {
    fn from(e: web3::ethabi::Error) -> Self {
        let msg = format!("rust_web3 abi error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::contract::Error> for CliError {
    fn from(e: web3::contract::Error) -> Self {
        let msg = format!("rust_web3 contract error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::contract::deploy::Error> for CliError {
    fn from(e: web3::contract::deploy::Error) -> Self {
        let msg = format!("rust_web3 contract deploy error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ethers::contract::AbiError> for CliError {
    fn from(e: ethers::contract::AbiError) -> Self {
        let msg = format!("ethers contract abi error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>>
    for CliError
{
    fn from(
        e: ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>,
    ) -> Self {
        let msg = format!("ethers contract call error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<WalletError> for CliError {
    fn from(e: WalletError) -> Self {
        let msg = format!("ethers contract wallet error: {:?}", e);
        Self::Internal(msg)
    }
}

impl
    From<
        ethers::contract::ContractError<
            ethers::middleware::SignerMiddleware<
                ethers::providers::Provider<ethers::providers::Http>,
                Wallet<ethers::core::k256::ecdsa::SigningKey>,
            >,
        >,
    > for CliError
{
    fn from(
        e: ethers::contract::ContractError<
            ethers::middleware::SignerMiddleware<
                ethers::providers::Provider<ethers::providers::Http>,
                Wallet<ethers::core::k256::ecdsa::SigningKey>,
            >,
        >,
    ) -> Self {
        match e {
            ethers::contract::ContractError::DecodingError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ethers::contract::ContractError::AbiError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ethers::contract::ContractError::DetokenizationError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ethers::contract::ContractError::MiddlewareError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ethers::contract::ContractError::ProviderError(e) => {
                let msg = format!("ethers contract sign error: {:?}", e);
                Self::Internal(msg)
            }
            ethers::contract::ContractError::ConstructorError => {
                let msg =
                    format!("ethers contract sign error: constructor is not defined in the ABI");
                Self::Internal(msg)
            }
            ethers::contract::ContractError::ContractNotDeployed => {
                let msg = format!("ethers contract sign error: Contract was not deployed");
                Self::Internal(msg)
            }
        }
    }
}

impl From<ethers::providers::ProviderError> for CliError {
    fn from(e: ethers::providers::ProviderError) -> Self {
        let msg = format!("ethers transaction error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        let msg = format!("io error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<reqwest::Error> for CliError {
    fn from(e: reqwest::Error) -> Self {
        let code = e.status().unwrap_or_default();
        let msg = format!("http error: {:?}", e);
        if code == StatusCode::from_u16(400).unwrap() {
            return Self::InvalidArgument(e.to_string());
        }
        if code == StatusCode::from_u16(404).unwrap() {
            return Self::NotFound;
        }

        Self::Internal(msg)
    }
}
