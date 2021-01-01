use num_bigint::{ BigInt };
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone)]
pub enum BencodexValue {
    Binary(Vec<u8>),
    Text(String),
    Boolean(bool),
    Number(BigInt),
    List(Vec<BencodexValue>),
    Dictionary(BTreeMap<BencodexKey, BencodexValue>),
    Null(()),
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Ord)]
pub enum BencodexKey {
    Binary(Vec<u8>),
    Text(String),
}
