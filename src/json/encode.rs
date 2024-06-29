use crate::{BencodexKey, BencodexValue};

fn to_json_impl(value: &BencodexValue, buf: &mut dyn std::io::Write) -> std::io::Result<()> {
    match value {
        BencodexValue::Binary(arg0) => buf.write_fmt(format_args!("{}", BencodexKey::from(arg0))),
        BencodexValue::Text(arg0) => buf.write_fmt(format_args!("{}", BencodexKey::from(arg0))),
        BencodexValue::Boolean(arg0) => buf
            .write_all(if *arg0 { b"true" } else { b"false" })
            .map(|_| ()),
        BencodexValue::Number(arg0) => write!(buf, "\"{}\"", arg0),
        BencodexValue::List(arg0) => {
            buf.write_all(b"[")?;
            for (i, item) in arg0.iter().enumerate() {
                to_json_impl(item, buf)?;
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
                write!(buf, "{}:{}", key, value)?;
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

pub fn to_json(value: &BencodexValue) -> String {
    let mut buf: Vec<u8> = vec![];
    to_json_impl(value, &mut buf).ok();

    String::from_utf8(buf).unwrap()
}
