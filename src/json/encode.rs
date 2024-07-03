use base64::Engine;

use crate::{BencodexKey, BencodexValue};

fn to_json_key_impl(
    value: &BencodexKey,
    options: &JsonEncodeOptions,
    buf: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    match value {
        BencodexKey::Binary(arg0) => match options.binary_encoding {
            BinaryEncoding::Base64 => buf.write_fmt(format_args!(
                "\"b64:{}\"",
                base64::engine::general_purpose::STANDARD.encode(arg0)
            )),
            BinaryEncoding::Hex => buf.write_fmt(format_args!("\"0x{}\"", hex::encode(arg0))),
        },
        BencodexKey::Text(arg0) => {
            buf.write_fmt(format_args!("\"\u{FEFF}{}\"", arg0.replace('\n', "\\n")))
        }
    }?;

    Ok(())
}

fn to_json_value_impl(
    value: &BencodexValue,
    options: &JsonEncodeOptions,
    buf: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    match value {
        BencodexValue::Binary(arg0) => to_json_key_impl(&BencodexKey::from(arg0), options, buf),
        BencodexValue::Text(arg0) => to_json_key_impl(&BencodexKey::from(arg0), options, buf),
        BencodexValue::Boolean(arg0) => buf
            .write_all(if *arg0 { b"true" } else { b"false" })
            .map(|_| ()),
        BencodexValue::Number(arg0) => write!(buf, "\"{}\"", arg0),
        BencodexValue::List(arg0) => {
            buf.write_all(b"[")?;
            for (i, item) in arg0.iter().enumerate() {
                to_json_value_impl(item, options, buf)?;
                if i < arg0.len() - 1 {
                    buf.write_all(b",")?;
                }
            }
            buf.write_all(b"]").map(|_| ())
        }
        BencodexValue::Dictionary(arg0) => {
            buf.write_all(b"{")?;
            let mut iter = arg0.iter().peekable();
            while let Some((key, value)) = iter.next() {
                to_json_key_impl(key, options, buf)?;
                buf.write_all(b":")?;
                to_json_value_impl(value, options, buf)?;
                if iter.peek().is_some() {
                    buf.write_all(b",")?;
                }
            }
            buf.write(b"}").map(|_| ())
        }
        BencodexValue::Null => buf.write(b"null").map(|_| ()),
    }?;

    Ok(())
}

pub enum BinaryEncoding {
    Base64,
    Hex,
}

impl Default for BinaryEncoding {
    fn default() -> Self {
        Self::Base64
    }
}

#[derive(Default)]
pub struct JsonEncodeOptions {
    pub binary_encoding: BinaryEncoding,
}

pub fn to_json(value: &BencodexValue) -> String {
    to_json_with_options(value, JsonEncodeOptions::default())
}

pub fn to_json_with_options(value: &BencodexValue, options: JsonEncodeOptions) -> String {
    let mut buf: Vec<u8> = vec![];
    to_json_value_impl(value, &options, &mut buf).ok();

    String::from_utf8(buf).unwrap()
}
