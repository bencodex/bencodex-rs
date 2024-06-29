use super::super::codec::utils;
#[cfg(test)]
use bencodex::to_json;

#[test]
fn spec_test() {
    let specs = utils::iter_spec().unwrap();
    for spec in specs {
        println!("---- SPEC [{}] ----", spec.name);

        println!("JSON: {:?}", spec.json);
        assert_eq!(to_json(&spec.bvalue), spec.json);

        println!("---- PASSED ----");
    }
}
