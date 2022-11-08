use aws_sdk_lambda::error::InvokeError;
use aws_sdk_lambda::types::SdkError;
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
        let msg = format!("web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::ethabi::Error> for CliError {
    fn from(e: web3::ethabi::Error) -> Self {
        let msg = format!("web3 abi error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::contract::Error> for CliError {
    fn from(e: web3::contract::Error) -> Self {
        let msg = format!("web3 contract error: {:?}", e);
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
