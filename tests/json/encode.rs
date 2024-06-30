use super::super::codec::utils;
#[cfg(test)]
use bencodex::{to_json_with_options, JsonOptions};

const SPEC_TEST_BASE64_OPTIONS: JsonOptions = JsonOptions {
    bytes_encode_method: bencodex::BytesEncodeMethod::Base64,
};

const SPEC_TEST_HEX_OPTIONS: JsonOptions = JsonOptions {
    bytes_encode_method: bencodex::BytesEncodeMethod::Hex,
};

#[test]
fn spec_test_base64() {
    let specs = utils::iter_spec(bencodex::BytesEncodeMethod::Base64).unwrap();
    for spec in specs {
        println!("---- SPEC [{}] ----", spec.name);

        println!("JSON: {:?}", spec.json);
        assert_eq!(
            to_json_with_options(&spec.bvalue, SPEC_TEST_BASE64_OPTIONS),
            spec.json
        );

        println!("---- PASSED ----");
    }
}

#[test]
fn spec_test_hex() {
    let specs = utils::iter_spec(bencodex::BytesEncodeMethod::Hex).unwrap();
    for spec in specs {
        println!("---- SPEC [{}] ----", spec.name);

        println!("JSON: {:?}", spec.json);
        assert_eq!(
            to_json_with_options(&spec.bvalue, SPEC_TEST_HEX_OPTIONS),
            spec.json
        );

        println!("---- PASSED ----");
    }
}
