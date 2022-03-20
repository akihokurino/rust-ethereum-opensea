use crate::error::CliResult;
use crate::{CliError, TEST_API_BASE_URL};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Method, Response, Url};
use web3::types::Address;

pub mod api;
pub mod erc1155;
pub mod erc721;
pub mod metadata;

fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}

#[derive(Clone, Debug)]
pub struct ApiClient {
    base_url: Url,
}

impl ApiClient {
    pub fn new() -> Self {
        let base_url = TEST_API_BASE_URL;
        ApiClient {
            base_url: base_url.parse().unwrap(),
        }
    }

    async fn call(&self, input: CallInput) -> CliResult<Response> {
        let mut url = self.base_url.clone();
        url.set_path(format!("{}", input.path).as_str());
        for q in input.query {
            url.query_pairs_mut()
                .append_pair(q.0.as_str(), q.1.as_str());
        }
        println!("call api: {}", url.to_string());

        let mut req = reqwest::Request::new(input.method, url);

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        *req.headers_mut() = headers;

        *req.body_mut() = input.body;

        let cli = reqwest::Client::new();
        let resp = cli.execute(req).await.map_err(|e| -> CliError {
            println!("error: {}", e.to_string());
            CliError::from(e)
        })?;

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
