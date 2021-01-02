use super::types::*;
use std::io;
use std::collections::BTreeMap;
use std::cmp::Ordering;

use itertools::Itertools;

use num_bigint::{ BigInt };

pub trait Encodable {
    fn encode(self, writer: &mut io::Write);
}

impl Encodable for Vec<u8> {
    fn encode(self, writer: &mut io::Write) {
        write!(writer, "{}:", self.len());
        writer.write(&self);
    }
}

impl Encodable for i64 {
    fn encode(self, writer: &mut io::Write) {
        write!(writer, "i{}e", self);
    }
}

impl Encodable for String {
    fn encode(self, writer: &mut io::Write) {
        let bytes = self.into_bytes();
        write!(writer, "u{}:", bytes.len());
        writer.write(&bytes);
    }
}

impl Encodable for bool {
    fn encode(self, writer: &mut io::Write) {
        writer.write(match self {
            true => &[b't'],
            false => &[b'f'],
        });
    }
}

impl Encodable for BigInt {
    fn encode(self, writer: &mut io::Write) {
        writer.write(&[b'i']);
        writer.write(&self.to_str_radix(10).into_bytes());
        writer.write(&[b'e']);
    }
}

impl Encodable for Vec<BencodexValue> {
    fn encode(self, writer: &mut io::Write) {
        writer.write(&[b'l']);
        for el in self {
            el.encode(writer);
        }
        writer.write(&[b'e']);
    }
}

impl Encodable for () {
    fn encode(self, writer: &mut io::Write) {
        writer.write(&[b'n']);
    }
}

fn encode_key(key: &BencodexKey) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    let (prefix, bytes) = match key {
        BencodexKey::Text(s) => (Some(vec![b'u']), s.to_owned().into_bytes()),
        BencodexKey::Binary(b) => (None as Option<Vec<u8>>, b.clone()),
    };
    match prefix {
        Some(p) => buf.extend(p),
        _ => (),
    };

    buf.extend(bytes.len().to_string().into_bytes());
    buf.push(b':');
    buf.extend(bytes);
    buf
}

impl Encodable for BencodexValue {
    fn encode(self, writer: &mut io::Write) {
        match self {
            BencodexValue::Binary(x) => x.encode(writer),
            BencodexValue::Text(x) => x.encode(writer),
            BencodexValue::Dictionary(x) => x.encode(writer),
            BencodexValue::List(x) => x.encode(writer),
            BencodexValue::Boolean(x) => x.encode(writer),
            BencodexValue::Null(x) => x.encode(writer),
            BencodexValue::Number(x) => x.encode(writer),
        };
    }
}

fn compare_vector<T: Ord>(xs: &Vec<T>, ys: &Vec<T>) -> Ordering {
    for (x, y) in xs.iter().zip(ys) {
        match x.cmp(&y) {
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        };
    }

    xs.len().cmp(&ys.len())
}

impl Encodable for BTreeMap<BencodexKey, BencodexValue> {
    fn encode(self, writer: &mut io::Write) {
        let pairs = self.into_iter()
            .map(|(key, value)| {
                let key_bytes = encode_key(&key);
                ( key, key_bytes, value ) 
            })
            .sorted_by(|(x_key, x_key_bytes, _), (y_key, y_key_bytes, _)| {
                match x_key {
                    BencodexKey::Text(_) => return Ordering::Greater,
                    _ => (),
                };

                match y_key {
                    BencodexKey::Text(_) => return Ordering::Less,
                    _ => (),
                };

                compare_vector(&x_key_bytes, &y_key_bytes)
            });

            writer.write(&[b'd']);
            for (_, key_bytes, value) in pairs {
                writer.write(&key_bytes);
                value.encode(writer);
            }
            writer.write(&[b'e']);
    }
}
