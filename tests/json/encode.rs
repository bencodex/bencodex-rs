use super::super::codec::utils;
#[cfg(test)]
use bencodex::{to_json_with_options, JsonOptions};

const SPEC_TEST_OPTIONS: JsonOptions = JsonOptions {
    bytes_encode_method: bencodex::BytesEncodeMethod::Base64,
};

#[test]
fn spec_test() {
    let specs = utils::iter_spec().unwrap();
    for spec in specs {
        println!("---- SPEC [{}] ----", spec.name);

        println!("JSON: {:?}", spec.json);
        assert_eq!(
            to_json_with_options(&spec.bvalue, SPEC_TEST_OPTIONS),
            spec.json
        );

        println!("---- PASSED ----");
    }
}
