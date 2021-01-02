use super::types::*;
use std::collections::BTreeMap;
use num_traits::ToPrimitive;
use std::str;
use std::str::FromStr;

use num_bigint::BigInt;

pub trait Decodable {
    fn decode(self) -> BencodexValue;
}

fn decode_impl(vector:& Vec<u8>, start: usize) -> (BencodexValue, usize) {
    match vector[start] {
        b'd' => {
            decode_dict_impl(vector, start)
        },
        b'l' => {
            decode_list_impl(vector, start)
        },
        b'u' => {
            decode_unicode_string_impl(vector, start)
        },
        b'i' => {
            decode_number_impl(vector, start)
        },
        b'0'..=b'9' => {
            decode_byte_string_impl(vector, start)
        },
        b't' => (BencodexValue::Boolean(true), 1),
        b'f' => (BencodexValue::Boolean(false), 1),
        b'n' => (BencodexValue::Null(()), 1),
        _ => todo!(),
    }
}

// start must be after 'd'
fn decode_dict_impl(vector: &Vec<u8>, start: usize) -> (BencodexValue, usize) {
    let mut tsize: usize = 1;
    let mut map = BTreeMap::new();
    while vector[start + tsize] != b'e' {
        if start + tsize >= vector.len() {
            todo!()
        }

        let index = start + tsize;
        let (value, size) = decode_impl(vector, index);
        tsize += size;
        let key = match value {
            BencodexValue::Text(s) => BencodexKey::Text(s),
            BencodexValue::Binary(b) => BencodexKey::Binary(b),
            _ => todo!(),
        };
        let (value, size) = decode_impl(vector, index);
        tsize += size;
        match map.insert(key, value) {
            None => (),
            Some(_) => todo!(),
        };
    };
    (BencodexValue::Dictionary(map), tsize + 1)
}

// start must be after 'l'
fn decode_list_impl(vector: &Vec<u8>, start: usize) -> (BencodexValue, usize) {
    let mut tsize: usize = 1;
    let mut list = Vec::new();
    while start + tsize < vector.len() && vector[start + tsize] != b'e' {
        let index = start + tsize;
        let (value, size) = decode_impl(vector, index);
        tsize += size;
        list.push(value);
    };
    (BencodexValue::List(list), tsize + 1)
}

// start must be after 'u'
fn decode_byte_string_impl(vector: &Vec<u8>, start: usize) -> (BencodexValue, usize) {
    let mut tsize: usize = 0;
    let (length, size) = match read_number(&vector[start + tsize..]) {
        None => todo!(),
        Some(v) => v,
    };
    tsize += size;

    if vector[start + tsize] != b':' {
        todo!()
    };
    tsize += 1;
    let lengthSize = length.to_usize().unwrap();
    (BencodexValue::Binary(vector[start + tsize..start + tsize + lengthSize].to_vec()), tsize + lengthSize)
}

// start must be after 'u'
fn decode_unicode_string_impl(vector: &Vec<u8>, start: usize) -> (BencodexValue, usize) {
    let mut tsize: usize = 1;
    let (length, size) = match read_number(&vector[start + tsize..]) {
        None => todo!(),
        Some(v) => v,
    };
    tsize += size;

    if vector[start + tsize] != b':' {
        todo!()
    };
    tsize += 1;
    let text = match str::from_utf8(&vector[start + tsize..start + tsize + length.to_usize().unwrap()]) {
        Ok(v) => v,
        Err(_) => todo!(),
    };
    tsize += length.to_usize().unwrap();
    (BencodexValue::Text(text.to_string()), tsize)
}

// start must be after 'i'
fn decode_number_impl(vector: &Vec<u8>, start: usize) -> (BencodexValue, usize) {
    let mut tsize: usize = 1;
    let (number, size) = match read_number(&vector[start + tsize..]) {
        None => todo!(),
        Some(v) => v,
    };
    tsize += size;

    if vector[start + tsize] != b'e' {
        todo!()
    } else {
        tsize += 1;
        (BencodexValue::Number(number), tsize)
    }
}

fn read_number(s: &[u8]) -> Option<(BigInt, usize)> {
    let mut size: usize = 0;
    loop {
        size += 1;
        match s[size] {
            b'0'..=b'9' => continue,
            _ => break,
        };
    };

    if size == 0 {
        None
    } else {
        Some((BigInt::from_str(&String::from_utf8(s[..size].to_vec()).unwrap()).unwrap(), size))
    }
}

impl Decodable for Vec<u8> {
    fn decode(self) -> BencodexValue {
        let (value, _) = decode_impl(&self, 0);
        value
    }
}
