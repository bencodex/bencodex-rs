#[cfg(test)]

use crate::bencodex::codec::encode::Encodable;
use super::utils;

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
