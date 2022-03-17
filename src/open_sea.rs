use web3::types::Address;

pub mod erc1155;
pub mod erc721;
pub mod metadata;

fn parse_address(address: String) -> Option<Address> {
    match address.trim_start_matches("0x").parse() {
        Ok(value) => Some(value),
        Err(_e) => None,
    }
}
