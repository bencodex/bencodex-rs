use num_bigint::{ BigInt };
use std::collections::BTreeMap;

#[derive(PartialEq)]
pub enum BencodexValue {
    Binary(Vec<u8>),
    Text(String),
    Boolean(bool),
    Number(BigInt),
    List(Vec<BencodexValue>),
    Dictionary(BTreeMap<BencodexKey, BencodexValue>),
    Null(()),
}

#[derive(PartialEq)]
pub enum BencodexKey {
    Binary(Vec<u8>),
    Text(String),
}
