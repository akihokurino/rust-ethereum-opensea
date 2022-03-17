use crate::aws::s3;
use crate::error::CliResult;

pub async fn exec() -> CliResult<()> {
    let s3_cli = s3::CLI::new();

    let base_url = s3_cli.create_bucket().await?;
    println!("{}", base_url);

    Ok(())
}
