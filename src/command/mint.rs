use crate::aws::s3;
use crate::error::CliResult;
use crate::open_sea::metadata::Metadata;
use crate::open_sea::*;
use crate::CliError;
use bytes::Bytes;
use std::fs::File;
use std::io::Read;

pub async fn erc721(
    name: String,
    description: String,
    image_url: String,
    image_filename: String,
    attrs: Vec<(String, String)>,
) -> CliResult<()> {
    if name.is_empty() || description.is_empty() || attrs.len() == 0 {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_url.is_empty() && image_filename.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let erc721_cli = erc721::CLI::new();
    let s3_cli = s3::CLI::new();

    let nft_image_url = if !image_filename.is_empty() {
        let mut file = File::open(format!("asset/{}", image_filename))?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf)?;

        let image_url = s3_cli
            .upload_object(
                format!("{}.png", name),
                Bytes::from(buf),
                "image/png".to_string(),
            )
            .await?;
        image_url
    } else {
        image_url
    };

    let metadata = Metadata::new(name.clone(), description, nft_image_url, attrs);
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
    image_filename: String,
    amount: u128,
    attrs: Vec<(String, String)>,
) -> CliResult<()> {
    if name.is_empty() || description.is_empty() || attrs.len() == 0 || amount <= 0 {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }
    if image_url.is_empty() && image_filename.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let erc1155_cli = erc1155::CLI::new();
    let s3_cli = s3::CLI::new();

    let nft_image_url = if !image_filename.is_empty() {
        let mut file = File::open(format!("asset/{}", image_filename))?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf)?;

        let image_url = s3_cli
            .upload_object(
                format!("{}.png", name),
                Bytes::from(buf),
                "image/png".to_string(),
            )
            .await?;
        image_url
    } else {
        image_url
    };

    let metadata = Metadata::new(name.clone(), description, nft_image_url, attrs);
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
