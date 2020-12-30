use num_bigint::{ BigInt };
use std::collections::BTreeMap;

pub enum BencodexValue {
    Binary(Vec<u8>),
    Text(String),
    Boolean(bool),
    Number(BigInt),
    List(Vec<BencodexValue>),
    Dictionary(BTreeMap<BencodexKey, BencodexValue>),
    Null(()),
}

pub enum BencodexKey {
    Binary(Vec<u8>),
    Text(String),
}
