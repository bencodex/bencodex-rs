use super::utils;
#[cfg(test)]
use bencodex::codec::decode::Decode;
use bencodex::json::BinaryEncoding;

#[test]
fn spec_test() {
    let specs = utils::iter_spec(BinaryEncoding::Base64).unwrap();
    for spec in specs {
        println!("---- SPEC [{}] ----", spec.name);
        println!("BVALUE: {:?}", spec.bvalue);
        let decoded = spec.encoded.decode().unwrap();
        assert_eq!(decoded, spec.bvalue);
        println!("---- PASSED ----");
    }
}
