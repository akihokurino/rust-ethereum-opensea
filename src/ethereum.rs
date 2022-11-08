pub mod erc1155;
pub mod erc721;

use web3::types::Address;

const GAS_LIMIT: i64 = 8500000;
const GAS_PRICE: i64 = 40000000000;

fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}
