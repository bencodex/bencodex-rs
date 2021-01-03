use super::types::*;
use itertools::Itertools;
use num_bigint::BigInt;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io;
use std::result::Result;

/// `Encode` is a trait to encode a [Bencodex] value.
///
/// [Bencodex]: https://bencodex.org/
pub trait Encode {
    /// Encode a [Bencodex] value from this type.
    ///
    /// If encoding succeeds, return [`Ok`]. Otherwise, it will pass [`std::io::Error`] occurred in inner logic.
    ///
    /// # Examples
    /// Basic usage with [`BencodexValue::Text`]:
    /// ```
    /// use bencodex::{ Encode, BencodexValue };
    ///
    /// let text = BencodexValue::Text("text".to_string());
    /// let mut vec = Vec::new();
    /// text.encode(&mut vec);
    ///
    /// assert_eq!(vec, vec![b'u', b'4', b':', b't', b'e', b'x', b't']);
    /// ```
    /// [Bencodex]: https://bencodex.org/
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error>;
}

impl Encode for Vec<u8> {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        match write!(writer, "{}:", self.len()) {
            Ok(()) => match writer.write(&self) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl Encode for i64 {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        write!(writer, "i{}e", self)
    }
}

impl Encode for String {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        let bytes = self.into_bytes();
        match write!(writer, "u{}:", bytes.len()) {
            Ok(()) => match writer.write(&bytes) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl Encode for bool {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        match writer.write(match self {
            true => &[b't'],
            false => &[b'f'],
        }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl Encode for BigInt {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        if let Err(e) = writer.write(&[b'i']) {
            return Err(e);
        }

        if let Err(e) = writer.write(&self.to_str_radix(10).into_bytes()) {
            return Err(e);
        }

        if let Err(e) = writer.write(&[b'e']) {
            return Err(e);
        }

        Ok(())
    }
}

impl Encode for Vec<BencodexValue> {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        if let Err(e) = writer.write(&[b'l']) {
            return Err(e);
        }

        for el in self {
            if let Err(e) = el.encode(writer) {
                return Err(e);
            }
        }

        if let Err(e) = writer.write(&[b'e']) {
            return Err(e);
        }

        Ok(())
    }
}

impl Encode for () {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        match writer.write(&[b'n']) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn encode_key(key: &BencodexKey) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![];
    let (prefix, bytes) = match key {
        BencodexKey::Text(s) => (Some(vec![b'u']), s.to_owned().into_bytes()),
        BencodexKey::Binary(b) => (None as Option<Vec<u8>>, b.clone()),
    };
    match prefix {
        Some(p) => buf.extend(p),
        _ => (),
    };

    buf.extend(bytes.len().to_string().into_bytes());
    buf.push(b':');
    buf.extend(bytes);
    buf
}

impl Encode for BencodexValue {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        // FIXME: rewrite more beautiful.
        match match self {
            BencodexValue::Binary(x) => x.encode(writer),
            BencodexValue::Text(x) => x.encode(writer),
            BencodexValue::Dictionary(x) => x.encode(writer),
            BencodexValue::List(x) => x.encode(writer),
            BencodexValue::Boolean(x) => x.encode(writer),
            BencodexValue::Null(x) => x.encode(writer),
            BencodexValue::Number(x) => x.encode(writer),
        } {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn compare_vector<T: Ord>(xs: &Vec<T>, ys: &Vec<T>) -> Ordering {
    for (x, y) in xs.iter().zip(ys) {
        match x.cmp(&y) {
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        };
    }

    xs.len().cmp(&ys.len())
}

impl Encode for BTreeMap<BencodexKey, BencodexValue> {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        let pairs = self
            .into_iter()
            .map(|(key, value)| {
                let key_bytes = encode_key(&key);
                (key, key_bytes, value)
            })
            .sorted_by(|(x_key, x_key_bytes, _), (y_key, y_key_bytes, _)| {
                match x_key {
                    BencodexKey::Text(_) => return Ordering::Greater,
                    _ => (),
                };

                match y_key {
                    BencodexKey::Text(_) => return Ordering::Less,
                    _ => (),
                };

                compare_vector(&x_key_bytes, &y_key_bytes)
            });

        if let Err(e) = writer.write(&[b'd']) {
            return Err(e);
        }

        for (_, key_bytes, value) in pairs {
            if let Err(e) = writer.write(&key_bytes) {
                return Err(e);
            }

            if let Err(e) = value.encode(writer) {
                return Err(e);
            }
        }

        if let Err(e) = writer.write(&[b'e']) {
            return Err(e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod compare_vector {
        use super::super::*;

        #[test]
        fn should_return_equal() {
            assert_eq!(Ordering::Equal, compare_vector(&Vec::<u8>::new(), &Vec::<u8>::new()));
            assert_eq!(Ordering::Equal, compare_vector(&vec![1, 2, 3], &vec![1, 2, 3]));
        }

        #[test]
        fn should_return_less() {
            assert_eq!(Ordering::Less, compare_vector(&vec![], &vec![3]));
            assert_eq!(Ordering::Less, compare_vector(&vec![0], &vec![1, 2, 3]));
            assert_eq!(Ordering::Less, compare_vector(&vec![1], &vec![9, 1, 1]));
            assert_eq!(Ordering::Less, compare_vector(&vec![1, 2], &vec![1, 2, 3]));
            assert_eq!(Ordering::Less, compare_vector(&vec![1, 9, 9], &vec![9, 1, 1]));
        }

        #[test]
        fn should_return_greater() {
            assert_eq!(Ordering::Greater, compare_vector(&vec![9], &vec![]));
            assert_eq!(Ordering::Greater, compare_vector(&vec![9], &vec![1, 2, 3]));
            assert_eq!(Ordering::Greater, compare_vector(&vec![1, 9, 2], &vec![1, 2, 2]));
        }
    }
}
