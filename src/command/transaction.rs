use crate::aws::lambda;
use crate::error::CliResult;
use crate::model::Schema;

pub async fn sell(
    contract_address: String,
    token_id: String,
    schema: Schema,
    ether: f64,
) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::sell(
        &contract_address,
        &token_id,
        &schema,
        ether,
    ))
    .await?;

    Ok(())
}

pub async fn transfer(
    contract_address: String,
    token_id: String,
    schema: Schema,
    to_address: String,
) -> CliResult<()> {
    lambda::invoke_open_sea_sdk(lambda::invoke_open_sea_sdk::Input::transfer(
        &contract_address,
        &token_id,
        &schema,
        &to_address,
    ))
    .await?;

    Ok(())
}
