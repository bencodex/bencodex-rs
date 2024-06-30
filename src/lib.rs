pub mod codec;

pub use codec::decode::{Decode, DecodeError};
pub use codec::encode::Encode;
pub use codec::types::{
    BencodexDictionary, BencodexKey, BencodexList, BencodexValue, BENCODEX_NULL,
};

mod json;
pub use json::encode::{to_json, to_json_with_options, BinaryEncoding, JsonOptions};
