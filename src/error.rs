use aws_sdk_s3::error::{CreateBucketError, PutObjectError};
use aws_sdk_s3::types::SdkError;
use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum CliError {
    #[error("invalid parameter error: {0}")]
    InvalidArgument(String),
    #[error("internal error: {0}")]
    Internal(String),
}

pub type CliResult<T> = Result<T, CliError>;

impl From<SdkError<CreateBucketError>> for CliError {
    fn from(e: SdkError<CreateBucketError>) -> Self {
        let msg = format!("s3 create bucket error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<PutObjectError>> for CliError {
    fn from(e: SdkError<PutObjectError>) -> Self {
        let msg = format!("s3 put object error: {:?}", e);
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
