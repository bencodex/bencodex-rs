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

fn compare_vector<T: Ord>(xs: &[T], ys: &[T]) -> Ordering {
    for (x, y) in xs.iter().zip(ys) {
        match x.cmp(&y) {
            Ordering::Equal => continue,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        };
    }

    xs.len().cmp(&ys.len())
}

fn compare_key(x: &BencodexKey, y: &BencodexKey) -> Ordering {
    match (x, y) {
        (BencodexKey::Text(x), BencodexKey::Text(y)) => compare_vector(x.as_bytes(), y.as_bytes()),
        (BencodexKey::Binary(x), BencodexKey::Binary(y)) => compare_vector(x, y),
        (BencodexKey::Text(_), BencodexKey::Binary(_)) => Ordering::Greater,
        (BencodexKey::Binary(_), BencodexKey::Text(_)) => Ordering::Less,
    }
}

impl Encode for BTreeMap<BencodexKey, BencodexValue> {
    fn encode(self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        let pairs = self
            .into_iter()
            .sorted_by(|(x, _), (y, _)| compare_key(x, y));

        if let Err(e) = writer.write(&[b'd']) {
            return Err(e);
        }

        for (key, value) in pairs {
            let key = match key {
                BencodexKey::Binary(x) => BencodexValue::Binary(x),
                BencodexKey::Text(x) => BencodexValue::Text(x),
            };

            if let Err(e) = key.encode(writer) {
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
    mod compare_key {
        use super::super::*;

        #[test]
        fn should_return_equal() {
            assert_eq!(
                Ordering::Equal,
                compare_key(
                    &BencodexKey::Text("foo".to_string()),
                    &BencodexKey::Text("foo".to_string())
                )
            );
            assert_eq!(
                Ordering::Equal,
                compare_key(
                    &BencodexKey::Binary(b"bar".to_vec()),
                    &BencodexKey::Binary(b"bar".to_vec())
                )
            );
        }

        #[test]
        fn should_return_greater() {
            assert_eq!(
                Ordering::Greater,
                compare_key(
                    &BencodexKey::Text("".to_string()),
                    &BencodexKey::Binary(b"".to_vec())
                )
            );
        }

        #[test]
        fn should_return_less() {
            assert_eq!(
                Ordering::Less,
                compare_key(
                    &BencodexKey::Binary(b"".to_vec()),
                    &BencodexKey::Text("".to_string())
                )
            );
        }
    }

    mod compare_vector {
        use super::super::*;

        #[test]
        fn should_return_equal() {
            assert_eq!(
                Ordering::Equal,
                compare_vector(&Vec::<u8>::new(), &Vec::<u8>::new())
            );
            assert_eq!(
                Ordering::Equal,
                compare_vector(&vec![1, 2, 3], &vec![1, 2, 3])
            );
        }

        #[test]
        fn should_return_less() {
            assert_eq!(Ordering::Less, compare_vector(&vec![], &vec![3]));
            assert_eq!(Ordering::Less, compare_vector(&vec![0], &vec![1, 2, 3]));
            assert_eq!(Ordering::Less, compare_vector(&vec![1], &vec![9, 1, 1]));
            assert_eq!(Ordering::Less, compare_vector(&vec![1, 2], &vec![1, 2, 3]));
            assert_eq!(
                Ordering::Less,
                compare_vector(&vec![1, 9, 9], &vec![9, 1, 1])
            );
        }

        #[test]
        fn should_return_greater() {
            assert_eq!(Ordering::Greater, compare_vector(&vec![9], &vec![]));
            assert_eq!(Ordering::Greater, compare_vector(&vec![9], &vec![1, 2, 3]));
            assert_eq!(
                Ordering::Greater,
                compare_vector(&vec![1, 9, 2], &vec![1, 2, 2])
            );
        }
    }

    mod encode {
        struct ConditionFailWriter {
            throw_counts: Vec<u64>,
            call_count: u64,
        }

        impl ConditionFailWriter {
            fn new(throw_counts: Vec<u64>) -> ConditionFailWriter {
                ConditionFailWriter {
                    throw_counts: throw_counts,
                    call_count: 0,
                }
            }
        }

        #[cfg(not(tarpaulin_include))]
        impl std::io::Write for ConditionFailWriter {
            fn write(&mut self, bytes: &[u8]) -> std::result::Result<usize, std::io::Error> {
                self.call_count += 1;
                if self.throw_counts.contains(&self.call_count) {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, ""))
                } else {
                    Ok(bytes.len())
                }
            }

            fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
                Ok(())
            }
        }

        mod null {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue = ();

                // write 'n'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod vec_u8 {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue = Vec::<u8>::new();

                // write length
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write 'e'
                let mut writer = ConditionFailWriter::new(vec![2]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write bytes
                let mut writer = ConditionFailWriter::new(vec![3]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod btree_map {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_order_keys() {
                let mut bvalue: BTreeMap<BencodexKey, BencodexValue> = BTreeMap::new();
                bvalue.insert(BencodexKey::Text("ua".to_string()), BencodexValue::Null(()));
                bvalue.insert(BencodexKey::Binary(vec![b'a']), BencodexValue::Null(()));
                bvalue.insert(BencodexKey::Text("ub".to_string()), BencodexValue::Null(()));
                bvalue.insert(BencodexKey::Binary(vec![b'b']), BencodexValue::Null(()));

                let mut writer = Vec::new();
                assert!(bvalue.to_owned().encode(&mut writer).is_ok());
                assert_eq!(b"d1:an1:bnu2:uanu2:ubne".to_vec(), writer);
            }

            #[test]
            fn should_pass_error() {
                let mut bvalue: BTreeMap<BencodexKey, BencodexValue> = BTreeMap::new();
                bvalue.insert(BencodexKey::Text("".to_string()), BencodexValue::Null(()));

                // write 'd'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write 'u' key prefix
                let mut writer = ConditionFailWriter::new(vec![2]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write '{}' key bytes length
                let mut writer = ConditionFailWriter::new(vec![3]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write ":" key delimeter
                let mut writer = ConditionFailWriter::new(vec![4]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write "" key bytes
                let mut writer = ConditionFailWriter::new(vec![5]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write value
                let mut writer = ConditionFailWriter::new(vec![6]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write 'e'
                let mut writer = ConditionFailWriter::new(vec![7]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod vec_bvalue {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue: &mut Vec<BencodexValue> = &mut Vec::new();
                bvalue.push(BencodexValue::Null(()));

                // write 'l'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write value
                let mut writer = ConditionFailWriter::new(vec![2]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write 'e'
                let mut writer = ConditionFailWriter::new(vec![3]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod string {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue: String = String::new();

                // write 'u'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write length
                let mut writer = ConditionFailWriter::new(vec![2]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write ':'
                let mut writer = ConditionFailWriter::new(vec![3]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write text
                let mut writer = ConditionFailWriter::new(vec![4]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod bool {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue = true;

                // write 't'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod big_int {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue = BigInt::from(0);

                // write 'i'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write number
                let mut writer = ConditionFailWriter::new(vec![2]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write 'e'
                let mut writer = ConditionFailWriter::new(vec![3]);
                let err = bvalue.to_owned().encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }

        mod i64 {
            use super::super::super::*;
            use super::*;

            #[test]
            fn should_pass_error() {
                let bvalue: i64 = 0;
                // write 'i'
                let mut writer = ConditionFailWriter::new(vec![1]);
                let err = bvalue.encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write number
                let mut writer = ConditionFailWriter::new(vec![2]);
                let err = bvalue.encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());

                // write 'e'
                let mut writer = ConditionFailWriter::new(vec![3]);
                let err = bvalue.encode(&mut writer).unwrap_err();
                assert_eq!(std::io::ErrorKind::Other, err.kind());
                assert_eq!("", err.to_string());
            }
        }
    }
}
