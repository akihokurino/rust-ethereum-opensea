use crate::error::{CliError, CliResult};
use aws_sdk_lambda::types::Blob;
use aws_sdk_lambda::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn invoke_open_sea_sdk(
    input: invoke_open_sea_sdk::Input,
) -> CliResult<invoke_open_sea_sdk::Output> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let arn = env::var("LAMBDA_OPENSEA_ARN").expect("should set lambda opensea arn");

    let json = serde_json::to_string(&input)?;
    let resp = client
        .invoke()
        .function_name(arn)
        .payload(Blob::new(json.into_bytes()))
        .send()
        .await?;

    let payload = resp.payload.unwrap();
    let payload = String::from_utf8(payload.into_inner()).ok().unwrap();
    let output: invoke_open_sea_sdk::Output = serde_json::from_str(&payload)?;

    if output.result != 0 {
        return Err(CliError::Internal(output.message));
    }

    Ok(output)
}

pub mod invoke_open_sea_sdk {
    use super::*;
    use crate::model::Schema;

    #[derive(Debug, Serialize)]
    pub struct Input {
        pub method: String,
        #[serde(rename(serialize = "walletAddress"))]
        pub wallet_address: String,
        #[serde(rename(serialize = "walletSecret"))]
        pub wallet_secret: String,
        #[serde(rename(serialize = "buyPayload"))]
        pub buy_payload: Option<BuyPayload>,
        #[serde(rename(serialize = "sellPayload"))]
        pub sell_payload: Option<SellPayload>,
        #[serde(rename(serialize = "transferPayload"))]
        pub transfer_payload: Option<TransferPayload>,
        #[serde(rename(serialize = "createMetadataPayload"))]
        pub create_metadata_payload: Option<CreateMetadataPayload>,
    }

    #[derive(Debug, Serialize)]
    pub struct BuyPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct SellPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
        #[serde(rename(serialize = "schemaName"))]
        pub schema_name: String,
        #[serde(rename(serialize = "ether"))]
        pub ether: f64,
        #[serde(rename(serialize = "quantity"))]
        pub quantity: i32,
    }

    #[derive(Debug, Serialize)]
    pub struct TransferPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
        #[serde(rename(serialize = "schemaName"))]
        pub schema_name: String,
        #[serde(rename(serialize = "transferAddress"))]
        pub transfer_address: String,
        #[serde(rename(serialize = "quantity"))]
        pub quantity: i32,
    }

    #[derive(Debug, Serialize)]
    pub struct CreateMetadataPayload {
        #[serde(rename(serialize = "name"))]
        pub name: String,
        #[serde(rename(serialize = "description"))]
        pub description: String,
        #[serde(rename(serialize = "externalUrl"))]
        pub external_url: String,
        #[serde(rename(serialize = "imageBase64"))]
        pub image_base64: String,
    }

    impl Input {
        pub fn sell(address: &String, token_id: &String, schema: &Schema, ether: f64) -> Self {
            let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
            let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

            Self {
                method: "sell".to_string(),
                wallet_address: wallet_address.to_owned(),
                wallet_secret: wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: Some(SellPayload {
                    token_address: address.to_owned().to_string(),
                    token_id: token_id.to_owned(),
                    schema_name: schema.to_owned().to_string(),
                    ether,
                    quantity: 1,
                }),
                transfer_payload: None,
                create_metadata_payload: None,
            }
        }

        pub fn transfer(
            address: &String,
            token_id: &String,
            schema: &Schema,
            to_address: &String,
        ) -> Self {
            let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
            let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

            Self {
                method: "transfer".to_string(),
                wallet_address: wallet_address.to_owned(),
                wallet_secret: wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: None,
                transfer_payload: Some(TransferPayload {
                    token_address: address.to_owned().to_string(),
                    token_id: token_id.to_owned(),
                    schema_name: schema.to_owned().to_string(),
                    transfer_address: to_address.to_owned().to_string(),
                    quantity: 1,
                }),
                create_metadata_payload: None,
            }
        }

        pub fn create_metadata(
            name: &String,
            description: &str,
            external_url: &str,
            image_base64: String,
        ) -> Self {
            let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
            let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

            Self {
                method: "createMetadata".to_string(),
                wallet_address: wallet_address.to_owned(),
                wallet_secret: wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: None,
                transfer_payload: None,
                create_metadata_payload: Some(CreateMetadataPayload {
                    name: name.to_owned(),
                    description: description.to_owned(),
                    external_url: external_url.to_owned(),
                    image_base64: image_base64.to_owned(),
                }),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Output {
        #[serde(rename(deserialize = "message"))]
        pub message: String,
        #[serde(rename(deserialize = "result"))]
        pub result: i32,
        #[serde(rename(deserialize = "ipfsResponse"))]
        pub ipfs_response: Option<OutputIPFS>,
    }

    #[derive(Debug, Deserialize)]
    pub struct OutputIPFS {
        #[serde(rename(deserialize = "hash"))]
        pub hash: String,
        #[serde(rename(deserialize = "url"))]
        pub url: String,
    }
}
