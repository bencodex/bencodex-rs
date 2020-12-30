#[cfg(test)]

use crate::bencodex::codec::encode::Encodable;
use num_bigint::BigInt;

#[test]
fn encode_number() {
    let mut buf: Vec<u8> = vec![];
    BigInt::from(0).encode(&mut buf);
    println!("{}", buf.len());
    assert_eq!(vec![b'i', b'0', b'e'], buf);
}
