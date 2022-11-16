use bigdecimal::BigDecimal;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Mul;
use std::str::FromStr;

pub mod ethers_rs;
pub mod rust_web3;

const GAS_LIMIT: i64 = 8500000;
const GAS_PRICE: i64 = 40000000000;

#[allow(unused)]
fn convert<'a>(value: &str, unit: &'a str) -> HashMap<&'a str, String> {
    let v = to_ether(value, unit);
    let mut map: HashMap<&'a str, String> = HashMap::new();

    map.insert(unit, BigDecimal::from_str(&value).unwrap().to_string());

    if unit != "wei" {
        map.insert("wei", s(&v, "1000000000000000000"));
    }
    if unit != "ether" {
        map.insert("ether", s(&v, "1"));
    }

    return map;
}

#[allow(unused)]
fn m(v: &BigDecimal, u: &str) -> BigDecimal {
    return v.mul(&BigDecimal::from_str(u).unwrap());
}

#[allow(unused)]
fn s(v: &BigDecimal, u: &str) -> String {
    return t(v.mul(&BigDecimal::from_str(u).unwrap()).to_string());
}

#[allow(unused)]
fn t(v: String) -> String {
    let re = Regex::new(r"(.*)\.0+$").unwrap();
    let v = re.replace_all(&v, "$1").to_string();
    let re = Regex::new(r"(.*\.\d+[1-9]+)(0+)$").unwrap();
    return re.replace_all(&v, "$1").to_string();
}

#[allow(unused)]
pub fn to_wei(value: &str, unit: &str) -> String {
    return convert(&value, &unit).get("wei").unwrap().to_string();
}

#[allow(unused)]
pub fn to_ether(value: &str, unit: &str) -> BigDecimal {
    let v = BigDecimal::from_str(&value).unwrap();

    if unit == "wei" {
        return m(&v, "0.000000000000000001");
    }
    if unit == "ether" {
        return m(&v, "1");
    }

    panic!("unit not supported");
}
