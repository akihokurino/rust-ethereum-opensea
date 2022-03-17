use crate::aws::s3;
use crate::error::CliResult;
use crate::open_sea::metadata::Metadata;
use crate::open_sea::*;
use crate::CliError;
use bytes::Bytes;

pub async fn erc721(
    name: String,
    description: String,
    image_url: String,
    attrs: Vec<(String, String)>,
) -> CliResult<()> {
    if name.is_empty() || description.is_empty() || image_url.is_empty() || attrs.len() == 0 {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let erc721_cli = erc721::CLI::new();
    let s3_cli = s3::CLI::new();

    let metadata = Metadata::new(name.clone(), description, image_url, attrs);
    let metadata = serde_json::to_string(&metadata)?;
    s3_cli
        .upload_object(
            format!("{}.metadata.json", name.clone()),
            Bytes::from(metadata),
            "application/json".to_string(),
        )
        .await?;

    erc721_cli.mint(name.clone()).await?;

    Ok(())
}

pub async fn erc1155(
    name: String,
    description: String,
    image_url: String,
    amount: u128,
    attrs: Vec<(String, String)>,
) -> CliResult<()> {
    if name.is_empty()
        || description.is_empty()
        || image_url.is_empty()
        || attrs.len() == 0
        || amount <= 0
    {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let erc1155_cli = erc1155::CLI::new();
    let s3_cli = s3::CLI::new();

    let metadata = Metadata::new(name.clone(), description, image_url, attrs);
    let metadata = serde_json::to_string(&metadata)?;
    s3_cli
        .upload_object(
            format!("{}.metadata.json", name.clone()),
            Bytes::from(metadata),
            "application/json".to_string(),
        )
        .await?;

    erc1155_cli.mint(name.clone(), amount).await?;

    Ok(())
}
