use crate::error::{CliError, CliResult};
use bytes::Bytes;
use reqwest::multipart;
use reqwest::multipart::Part;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Clone, Debug)]
pub struct Adapter {
    base_url: Url,
    key: String,
    secret: String,
}

impl Adapter {
    pub fn new() -> Self {
        let base_url = env::var("IPFS_URL").expect("IPFS_URL must be set");
        let key = env::var("IPFS_KEY").expect("IPFS_KEY must be set");
        let secret = env::var("IPFS_SECRET").expect("IPFS_SECRET must be set");

        Adapter {
            base_url: base_url.parse().unwrap(),
            key,
            secret,
        }
    }

    pub async fn upload(&self, byte: Bytes, name: String) -> CliResult<IpfsOutput> {
        let form = multipart::Form::new().part("file", Part::bytes(byte.to_vec()).file_name(name));

        let mut url = self.base_url.to_owned();
        url.set_path("/api/v0/add");

        let resp = reqwest::Client::new()
            .post(url.to_string())
            .multipart(form)
            .basic_auth(&self.key, Some(&self.secret))
            .send()
            .await?
            .error_for_status()?
            .json::<IpfsOutput>()
            .await
            .map_err(CliError::from)?;

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsOutput {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Hash")]
    pub hash: String,
}
