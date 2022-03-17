use crate::error::CliResult;
use aws_sdk_s3::model::{BucketCannedAcl, BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use std::env;

#[derive(Clone, Debug)]
pub struct CLI {
    bucket: String,
    region: String,
}

impl CLI {
    pub fn new() -> Self {
        let bucket = env::var("AWS_BUCKET").expect("AWS_BUCKET must be set");
        let region = env::var("AWS_REGION").expect("AWS_REGION must be set");

        CLI { bucket, region }
    }

    pub async fn create_bucket(&self) -> CliResult<String> {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        let constraint = BucketLocationConstraint::from(self.region.clone().as_str());
        let cfg = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .acl(BucketCannedAcl::PublicRead)
            .bucket(self.bucket.clone())
            .send()
            .await?;

        Ok(format!(
            "https://{}.s3.{}.amazonaws.com",
            self.bucket.clone(),
            self.region.clone()
        ))
    }

    pub async fn upload_object(
        &self,
        key: String,
        data: Bytes,
        content_type: String,
    ) -> CliResult<String> {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        let body = ByteStream::from(data);

        client
            .put_object()
            .bucket(self.bucket.clone())
            .key(&key)
            .content_type(content_type)
            .body(body)
            .send()
            .await?;

        Ok(format!(
            "https://{}.s3.ap-northeast-1.amazonaws.com/{}",
            self.bucket.clone(),
            key
        ))
    }
}
