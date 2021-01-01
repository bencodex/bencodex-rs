#[cfg(test)]

use crate::bencodex::codec::encode::Encodable;
use super::utils;
use num_bigint::BigInt;

#[test]
#[allow(dead_code)]
fn encode_number() {
    let mut buf: Vec<u8> = vec![];
    BigInt::from(0).encode(&mut buf);
    println!("{}", buf.len());    
    assert_eq!(vec![b'i', b'0', b'e'], buf);
}

#[test]
fn spec_test() {
    let specs = utils::iter_spec().unwrap();
    for spec in specs {
        let mut buf: Vec<u8> = vec![];
        println!("---- SPEC [{}] ----", spec.name);
        println!("BVALUE: {:?}", spec.bvalue);
        spec.bvalue.encode(&mut buf);
        assert_eq!(buf, spec.encoded);
        println!("---- PASSED ----");
    }
}
