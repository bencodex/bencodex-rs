use std::result::Result;

use base64::Engine;
use itertools::Itertools;
use serde_json::Value;

use crate::{BencodexDictionary, BencodexKey, BencodexValue};

#[derive(Debug, PartialEq)]
pub enum JsonDecodeError {
    InvalidJsonString,
    InvalidJson,
}

fn from_json_key_impl(s: &str) -> Result<BencodexKey, JsonDecodeError> {
    if let Some(rest) = s.strip_prefix("b64:") {
        let binary = base64::engine::general_purpose::STANDARD
            .decode(rest)
            .map_err(|_| JsonDecodeError::InvalidJson)?;
        Ok(BencodexKey::Binary(binary))
    } else if let Some(rest) = s.strip_prefix("0x") {
        let binary = hex::decode(rest).map_err(|_| JsonDecodeError::InvalidJson)?;
        Ok(BencodexKey::Binary(binary))
    } else if let Some(rest) = s.strip_prefix('\u{FEFF}') {
        Ok(BencodexKey::Text(rest.to_string()))
    } else {
        Err(JsonDecodeError::InvalidJson)
    }
}

pub fn from_json(value: &Value) -> Result<BencodexValue, JsonDecodeError> {
    match value {
        Value::Null => Ok(BencodexValue::Null),
        Value::Bool(b) => Ok(BencodexValue::Boolean(*b)),
        Value::Number(_) => Err(JsonDecodeError::InvalidJson),
        Value::Array(arr) => Ok(BencodexValue::List(
            arr.iter().map(from_json).try_collect()?,
        )),
        Value::Object(obj) => {
            let mut map = BencodexDictionary::new();
            for (k, v) in obj {
                map.insert(from_json_key_impl(k)?, from_json(v)?);
            }

            Ok(BencodexValue::Dictionary(map))
        }
        Value::String(s) => {
            if let Ok(key) = from_json_key_impl(s) {
                Ok(match key {
                    BencodexKey::Text(t) => BencodexValue::Text(t.to_owned()),
                    BencodexKey::Binary(b) => BencodexValue::Binary(b.to_owned()),
                })
            } else if s
                .as_bytes()
                .iter()
                .all(|x| x.is_ascii_digit() || *x == b'-')
            {
                Ok(BencodexValue::Number(
                    s.parse().map_err(|_| JsonDecodeError::InvalidJson)?,
                ))
            } else {
                Err(JsonDecodeError::InvalidJson)
            }
        }
    }
}

pub fn from_json_string(s: &str) -> Result<BencodexValue, JsonDecodeError> {
    let json = serde_json::from_str(s).map_err(|_| JsonDecodeError::InvalidJsonString)?;
    from_json(&json)
}
