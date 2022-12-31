use crate::error::CliResult;
use crate::open_sea::{ApiClient, CallInput};
use crate::CliError;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub enum OrderSide {
    Buy,
    Sell,
}

impl ApiClient {
    pub async fn get_asset(&self, input: get_asset::Input) -> CliResult<get_asset::Output> {
        #[derive(Debug, Serialize)]
        struct Body {}

        let body = Body {};
        let query = vec![];

        self.call(CallInput {
            method: Method::GET,
            path: format!(
                "/api/v1/asset/{}/{}",
                input.contract_address, input.token_id
            )
            .to_string(),
            body: Some(
                serde_json::to_string(&body)
                    .map_err(|e| CliError::Internal(e.to_string()))?
                    .into(),
            ),
            query,
        })
        .await?
        .error_for_status()?
        .json::<get_asset::Output>()
        .await
        .map_err(CliError::from)
    }

    pub async fn get_order(&self, input: get_order::Input) -> CliResult<get_order::Output> {
        #[derive(Debug, Serialize)]
        struct Body {}

        let body = Body {};
        let _side = match input.side {
            OrderSide::Buy => "0",
            OrderSide::Sell => "1",
        };
        let query = vec![
            ("limit".to_string(), "1".to_string()),
            // ("side".to_string(), side.to_string()),
            ("asset_contract_address".to_string(), input.contract_address),
            ("token_ids".to_string(), input.token_id),
        ];

        self.call(CallInput {
            method: Method::GET,
            path: "/v2/orders/goerli/seaport/listings".to_string(),
            body: Some(
                serde_json::to_string(&body)
                    .map_err(|e| CliError::Internal(e.to_string()))?
                    .into(),
            ),
            query,
        })
        .await?
        .error_for_status()?
        .json::<get_order::Output>()
        .await
        .map_err(CliError::from)
    }
}

pub mod get_asset {
    use super::*;

    pub struct Input {
        pub contract_address: String,
        pub token_id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub id: i32,
        pub image_url: String,
        pub image_preview_url: String,
        pub name: String,
        pub description: String,
        pub asset_contract: AssetContract,
        pub permalink: String,
        pub collection: Collection,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssetContract {
        pub address: String,
        pub asset_contract_type: String,
        pub name: String,
        pub schema_name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Collection {
        pub payment_tokens: Vec<PaymentToken>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PaymentToken {
        pub symbol: Option<String>,
        pub eth_price: Option<f64>,
        pub usd_price: Option<f64>,
    }
}

pub mod get_order {
    use super::*;

    pub struct Input {
        pub side: OrderSide,
        pub contract_address: String,
        pub token_id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub orders: Vec<Order>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        pub order_hash: String,
        pub side: String,
        pub current_price: String,
        pub created_date: String,
        pub closing_date: String,
    }
}
