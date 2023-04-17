use crate::open_sea::api::OrderSide;
use crate::{CliResult, Error};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Method, Response, Url};
use std::env;

pub mod api;
pub mod metadata;

#[derive(Clone, Debug)]
pub struct ApiClient {
    base_url: Url,
}

impl ApiClient {
    pub fn new() -> Self {
        let base_url = env::var("OPENSEA_API_BASE_URL").expect("should set opensea base url");
        ApiClient {
            base_url: base_url.parse().unwrap(),
        }
    }

    async fn call(&self, input: CallInput) -> CliResult<Response> {
        let mut url = self.base_url.to_owned();
        url.set_path(format!("{}", input.path).as_str());
        for q in input.query {
            url.query_pairs_mut()
                .append_pair(q.0.as_str(), q.1.as_str());
        }

        let mut req = reqwest::Request::new(input.method, url);

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        *req.headers_mut() = headers;

        *req.body_mut() = input.body;

        let cli = reqwest::Client::new();
        let resp = cli
            .execute(req)
            .await
            .map_err(|e| -> Error { Error::from(e) })?;

        Ok(resp)
    }
}

#[derive(Default)]
pub struct CallInput {
    pub method: Method,
    pub path: String,
    pub body: Option<Body>,
    pub query: Vec<(String, String)>,
}

pub async fn show_asset(contract_address: String, token_id: String) -> CliResult<()> {
    if contract_address.is_empty() || token_id.is_empty() {
        return Err(Error::InvalidArgument("parameter is invalid".to_string()));
    }

    let api_cli = ApiClient::new();
    let asset = api_cli
        .get_asset(api::get_asset::Input {
            contract_address,
            token_id,
        })
        .await?;

    println!("{:?}", asset);

    Ok(())
}

pub async fn show_order(
    contract_address: String,
    token_id: String,
    side: OrderSide,
) -> CliResult<()> {
    if contract_address.is_empty() || token_id.is_empty() {
        return Err(Error::InvalidArgument("parameter is invalid".to_string()));
    }

    let api_cli = ApiClient::new();
    let order = api_cli
        .get_order(api::get_order::Input {
            side,
            contract_address,
            token_id,
        })
        .await?;

    if order.orders.len() == 0 {
        return Err(Error::NotFound);
    }

    println!("{:?}", order.orders.first().unwrap());

    Ok(())
}
