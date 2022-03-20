use crate::error::CliResult;
use crate::open_sea::api::OrderSide;
use crate::open_sea::*;
use crate::CliError;

pub async fn buy(contract_address: String, token_id: String) -> CliResult<()> {
    if contract_address.is_empty() || token_id.is_empty() {
        return Err(CliError::InvalidArgument(
            "parameter is invalid".to_string(),
        ));
    }

    let api_cli = ApiClient::new();
    let order = api_cli
        .get_order(api::get_order::Input {
            side: OrderSide::Sell,
            contract_address,
            token_id,
        })
        .await?;

    if order.orders.len() == 0 {
        return Err(CliError::NotFound);
    }

    let order = order.orders.first().unwrap();

    Ok(())
}
