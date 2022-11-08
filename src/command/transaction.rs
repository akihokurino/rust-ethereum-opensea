use crate::aws::lambda;
use crate::error::CliResult;
use crate::model::Schema;

pub async fn sell(token_id: String, schema: Schema, ether: f64) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::sell(
        &schema.address(),
        &token_id,
        &schema,
        ether,
    ))
    .await?;

    Ok(())
}

pub async fn transfer(token_id: String, schema: Schema, to_address: String) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::transfer(
        &schema.address(),
        &token_id,
        &schema,
        &to_address,
    ))
    .await?;

    Ok(())
}
