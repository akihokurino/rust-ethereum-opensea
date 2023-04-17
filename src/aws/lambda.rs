use crate::{CliResult, Error};
use aws_sdk_lambda::types::Blob;
use aws_sdk_lambda::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn invoke_open_sea_sdk(input: invoke_open_sea_sdk::Input) -> CliResult<()> {
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
        return Err(Error::Internal(output.message));
    }

    println!("lambda output: {:?}", output);

    Ok(())
}

pub mod invoke_open_sea_sdk {
    use super::*;
    use crate::Schema;

    #[derive(Debug, Serialize)]
    pub struct Input {
        pub method: String,
        #[serde(rename(serialize = "walletAddress"))]
        pub wallet_address: String,
        #[serde(rename(serialize = "walletSecret"))]
        pub wallet_secret: String,
        #[serde(rename(serialize = "sellPayload"))]
        pub sell_payload: Option<SellPayload>,
    }

    #[derive(Debug, Serialize)]
    pub struct SellPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
        #[serde(rename(serialize = "schemaName"))]
        pub schema_name: String,
        #[serde(rename(serialize = "ethers_rs"))]
        pub ether: f64,
        #[serde(rename(serialize = "quantity"))]
        pub quantity: i32,
    }

    impl Input {
        pub fn sell(address: String, token_id: String, schema: Schema, ether: f64) -> Self {
            let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
            let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

            Self {
                method: "sell".to_string(),
                wallet_address: wallet_address.to_owned(),
                wallet_secret: wallet_secret.to_owned(),
                sell_payload: Some(SellPayload {
                    token_address: address.to_owned().to_string(),
                    token_id: token_id.to_owned(),
                    schema_name: schema.to_owned().to_string(),
                    ether,
                    quantity: 1,
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
    }
}
