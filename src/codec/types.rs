use num_bigint::BigInt;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone)]
pub enum BencodexValue {
    Binary(Vec<u8>),
    Text(String),
    Boolean(bool),
    Number(BigInt),
    List(Vec<BencodexValue>),
    Dictionary(BTreeMap<BencodexKey, BencodexValue>),
    Null(()),
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Ord)]
pub enum BencodexKey {
    Binary(Vec<u8>),
    Text(String),
}

impl Into<BencodexKey> for &str {
    fn into(self) -> BencodexKey {
        BencodexKey::Text(self.to_string())
    }
}

impl Into<BencodexKey> for String {
    fn into(self) -> BencodexKey {
        BencodexKey::Text(self)
    }
}

impl Into<BencodexKey> for Vec<u8> {
    fn into(self) -> BencodexKey {
        BencodexKey::Binary(self)
    }
}

impl Into<BencodexKey> for &[u8] {
    fn into(self) -> BencodexKey {
        BencodexKey::Binary(self.to_vec())
    }
}

impl Into<BencodexValue> for &[u8] {
    fn into(self) -> BencodexValue {
        BencodexValue::Binary(self.to_vec())
    }
}

impl Into<BencodexValue> for Vec<u8> {
    fn into(self) -> BencodexValue {
        BencodexValue::Binary(self)
    }
}

impl Into<BencodexValue> for &str {
    fn into(self) -> BencodexValue {
        BencodexValue::Text(self.to_string())
    }
}

impl Into<BencodexValue> for String {
    fn into(self) -> BencodexValue {
        BencodexValue::Text(self)
    }
}

macro_rules! bencodex_value_number_impl {
    ($x:tt) => {
        impl Into<BencodexValue> for $x {
            fn into(self) -> BencodexValue {
                BencodexValue::Number(self.into())
            }
        }
    };
}

bencodex_value_number_impl!(u16);
bencodex_value_number_impl!(u32);
bencodex_value_number_impl!(u64);
bencodex_value_number_impl!(i8);
bencodex_value_number_impl!(i16);
bencodex_value_number_impl!(i32);
bencodex_value_number_impl!(i64);

impl Into<BencodexValue> for bool {
    fn into(self) -> BencodexValue {
        BencodexValue::Boolean(self)
    }
}

impl<T> Into<BencodexValue> for Vec<T>
where
    T: Into<BencodexValue>,
{
    fn into(self) -> BencodexValue {
        let mut vec = Vec::new();
        for v in self {
            vec.push(v.into());
        }

        BencodexValue::List(vec)
    }
}

impl<T, U> Into<BencodexValue> for BTreeMap<T, U>
where
    T: Into<BencodexKey>,
    U: Into<BencodexValue>,
{
    fn into(self) -> BencodexValue {
        let mut map = BTreeMap::<BencodexKey, BencodexValue>::new();
        for (key, value) in self {
            map.insert(key.into(), value.into());
        }

        BencodexValue::Dictionary(map)
    }
}

impl Into<BencodexValue> for () {
    fn into(self) -> BencodexValue {
        BencodexValue::Null(self)
    }
}

#[cfg(test)]
mod tests {
    mod into {
        use std::array::IntoIter;
        use std::{collections::BTreeMap, iter::FromIterator};

        use super::super::{BencodexKey, BencodexValue};

        #[test]
        fn text() {
            let s: &str = "value";
            let value: BencodexKey = s.into();
            assert_eq!(value, BencodexKey::Text("value".to_string()));

            let s: String = "value".to_string();
            let value: BencodexKey = s.into();
            assert_eq!(value, BencodexKey::Text("value".to_string()));

            let s: &str = "value";
            let value: BencodexValue = s.into();
            assert_eq!(value, BencodexValue::Text("value".to_string()));

            let s: String = "value".to_string();
            let value: BencodexValue = s.into();
            assert_eq!(value, BencodexValue::Text("value".to_string()));
        }

        #[test]
        fn binary() {
            let b: &[u8] = &[0, 1, 2, 3];
            let value: BencodexKey = b.into();
            assert_eq!(value, BencodexKey::Binary(vec![0, 1, 2, 3]));

            let b: Vec<u8> = vec![0, 1, 2, 3];
            let value: BencodexKey = b.into();
            assert_eq!(value, BencodexKey::Binary(vec![0, 1, 2, 3]));

            let b: &[u8] = &[0, 1, 2, 3];
            let value: BencodexValue = b.into();
            assert_eq!(value, BencodexValue::Binary(vec![0, 1, 2, 3]));

            let b: Vec<u8> = vec![0, 1, 2, 3];
            let value: BencodexValue = b.into();
            assert_eq!(value, BencodexValue::Binary(vec![0, 1, 2, 3]));
        }

        #[test]
        fn number() {
            let n: u16 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));

            let n: u32 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));

            let n: u64 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));

            let n: i8 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));

            let n: i16 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));

            let n: i32 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));

            let n: i64 = 0;
            let value: BencodexValue = n.into();
            assert_eq!(value, BencodexValue::Number(0.into()));
        }

        #[test]
        fn boolean() {
            let value: BencodexValue = true.into();
            assert_eq!(value, BencodexValue::Boolean(true));

            let value: BencodexValue = false.into();
            assert_eq!(value, BencodexValue::Boolean(false));
        }

        #[test]
        fn null() {
            let value: BencodexValue = ().into();
            assert_eq!(value, BencodexValue::Null(()));
        }

        #[test]
        fn list() {
            let l = vec!["A", "B", "C", "D"];
            let value: BencodexValue = l.into();
            assert_eq!(
                value,
                BencodexValue::List(vec!["A".into(), "B".into(), "C".into(), "D".into()])
            );

            let l = vec![0, 1, 2, 3];
            let value: BencodexValue = l.into();
            assert_eq!(
                value,
                BencodexValue::List(vec![0.into(), 1.into(), 2.into(), 3.into()])
            );

            let l = vec![(), (), ()];
            let value: BencodexValue = l.into();
            assert_eq!(
                value,
                BencodexValue::List(vec![().into(), ().into(), ().into()])
            );

            let l: Vec<Vec<u8>> = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]];
            let value: BencodexValue = l.into();
            assert_eq!(
                value,
                BencodexValue::List(vec![vec![0u8, 1, 2, 3].into(), vec![4u8, 5, 6, 7].into(),])
            );
        }

        #[test]
        fn dictionary() {
            let mut map = BTreeMap::<String, &[u8]>::new();
            map.insert("foo".to_string(), b"bar");
            let actual: BencodexValue = map.into();

            let expected = BencodexValue::Dictionary(BTreeMap::from_iter(IntoIter::new([(
                BencodexKey::Text("foo".to_string()),
                BencodexValue::Binary(vec![b'b', b'a', b'r']),
            )])));

            assert_eq!(actual, expected);
        }
    }
}
