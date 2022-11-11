pub mod rust_token1155;
pub mod rust_token721;

use web3::types::Address;

fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}
