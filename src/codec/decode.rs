use super::types::*;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::result::Result;
use std::str;
use std::str::FromStr;

/// The enum type which describes why [`DecodeError`] occurred.
#[derive(Debug, PartialEq)]
pub enum DecodeErrorReason {
    /// This should be used when it failed to decode. In future, it will be separated more and more.
    InvalidBencodexValue,
    /// This should be used when it failed to decode because there is unexpected token appeared while decoding.
    ///
    /// # Example
    ///
    /// For example, The encoded bytes of [`BencodexValue::Number`] are formed as 'i{}e' (e.g., 'i0e', 'i2147483647e'). If it is not satisified, it should be result through inside [`Err`].
    ///
    /// ```
    /// use bencodex::{ Decode, DecodeError, DecodeErrorReason };
    ///
    /// //                     v -- should be b'0' ~ b'9' digit.
    /// let vec = vec![b'i', b':', b'e'];
    /// let error = vec.decode().unwrap_err().reason;
    /// let expected_error = DecodeErrorReason::UnexpectedToken {
    ///     token: b':',
    ///     point: 1,
    /// };
    /// assert_eq!(expected_error, error);
    /// ```
    UnexpectedToken { token: u8, point: usize },
}

/// The error type which is returned from decoding a Bencodex value through [`Decode::decode`].
#[derive(Debug, PartialEq)]
pub struct DecodeError {
    pub reason: DecodeErrorReason,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DecodeError (reason: {:?})", self.reason)
    }
}

impl Error for DecodeError {}

/// `Decode` is a trait to decode a [Bencodex] value.
///
/// [Bencodex]: https://bencodex.org/
pub trait Decode {
    /// Decodes a [Bencodex] value to return from this type.
    ///
    /// If decoding succeeds, return the value inside [`Ok`]. Otherwise, return the [`DecodeError`] with [`DecodeErrorReason`] inside [`Err`].
    ///
    /// # Examples
    /// Basic usage with [`Vec<u8>`], the default implementor which implements `Decode`.
    /// ```
    /// use bencodex::{ Decode, BencodexValue };
    ///
    /// let vec = vec![b'n'];
    /// let null = vec.decode().unwrap();
    ///
    /// assert_eq!(BencodexValue::Null(()), null);
    /// ```
    /// [Bencodex]: https://bencodex.org/
    fn decode(self) -> Result<BencodexValue, DecodeError>;
}

fn decode_impl(vector: &Vec<u8>, start: usize) -> Result<(BencodexValue, usize), DecodeError> {
    if start >= vector.len() {
        return Err(DecodeError {
            reason: DecodeErrorReason::InvalidBencodexValue,
        });
    }

    match vector[start] {
        b'd' => decode_dict_impl(vector, start),
        b'l' => decode_list_impl(vector, start),
        b'u' => decode_unicode_string_impl(vector, start),
        b'i' => decode_number_impl(vector, start),
        b'0'..=b'9' => decode_byte_string_impl(vector, start),
        b't' => Ok((BencodexValue::Boolean(true), 1)),
        b'f' => Ok((BencodexValue::Boolean(false), 1)),
        b'n' => Ok((BencodexValue::Null(()), 1)),
        _ => Err(DecodeError {
            reason: DecodeErrorReason::UnexpectedToken {
                token: vector[start],
                point: start,
            },
        }),
    }
}

// start must be on 'd'
fn decode_dict_impl(vector: &Vec<u8>, start: usize) -> Result<(BencodexValue, usize), DecodeError> {
    let mut tsize: usize = 1;
    let mut map = BTreeMap::new();
    while vector[start + tsize] != b'e' {
        if start + tsize >= vector.len() {
            return Err(DecodeError {
                reason: DecodeErrorReason::InvalidBencodexValue,
            });
        }

        let index = start + tsize;
        let (value, size) = match decode_impl(vector, index) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        tsize += size;
        let key = match value {
            BencodexValue::Text(s) => BencodexKey::Text(s),
            BencodexValue::Binary(b) => BencodexKey::Binary(b),
            _ => {
                return Err(DecodeError {
                    reason: DecodeErrorReason::InvalidBencodexValue,
                })
            }
        };
        let index = start + tsize;
        let (value, size) = match decode_impl(vector, index) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        tsize += size;
        match map.insert(key, value) {
            None => (),
            Some(_) => todo!(),
        };
    }
    Ok((BencodexValue::Dictionary(map), tsize + 1))
}

// start must be on 'l'
fn decode_list_impl(vector: &Vec<u8>, start: usize) -> Result<(BencodexValue, usize), DecodeError> {
    let mut tsize: usize = 1;
    let mut list = Vec::new();
    while start + tsize < vector.len() && vector[start + tsize] != b'e' {
        let index = start + tsize;
        let (value, size) = match decode_impl(vector, index) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        tsize += size;
        list.push(value);
    }

    Ok((BencodexValue::List(list), tsize + 1))
}

fn decode_byte_string_impl(
    vector: &Vec<u8>,
    start: usize,
) -> Result<(BencodexValue, usize), DecodeError> {
    let mut tsize: usize = 0;
    let (length, size) = match read_number(&vector[start + tsize..]) {
        None => {
            return Err(DecodeError {
                reason: DecodeErrorReason::InvalidBencodexValue,
            })
        }
        Some(v) => v,
    };
    tsize += size;

    if vector[start + tsize] != b':' {
        return Err(DecodeError {
            reason: DecodeErrorReason::UnexpectedToken {
                token: vector[start + tsize],
                point: start + tsize,
            },
        });
    };
    tsize += 1;
    let length_size = length.to_usize().unwrap();
    Ok((
        BencodexValue::Binary(vector[start + tsize..start + tsize + length_size].to_vec()),
        tsize + length_size,
    ))
}

// start must be on 'u'
fn decode_unicode_string_impl(
    vector: &Vec<u8>,
    start: usize,
) -> Result<(BencodexValue, usize), DecodeError> {
    let mut tsize: usize = 1;
    let (length, size) = match read_number(&vector[start + tsize..]) {
        None => {
            return Err(DecodeError {
                reason: DecodeErrorReason::InvalidBencodexValue,
            })
        }
        Some(v) => v,
    };
    tsize += size;

    if vector[start + tsize] != b':' {
        return Err(DecodeError {
            reason: DecodeErrorReason::UnexpectedToken {
                token: vector[start + tsize],
                point: start + tsize,
            },
        });
    };

    tsize += 1;
    let length_size = length.to_usize().unwrap();
    let text = match str::from_utf8(&vector[start + tsize..start + tsize + length_size]) {
        Ok(v) => v,
        Err(e) => {
            return Err(DecodeError {
                reason: DecodeErrorReason::InvalidBencodexValue,
            })
        }
    };
    tsize += length_size;
    Ok((BencodexValue::Text(text.to_string()), tsize))
}

// start must be on 'i'
fn decode_number_impl(
    vector: &Vec<u8>,
    start: usize,
) -> Result<(BencodexValue, usize), DecodeError> {
    let mut tsize: usize = 1;
    let (number, size) = match read_number(&vector[start + tsize..]) {
        None => {
            return Err(DecodeError {
                reason: DecodeErrorReason::UnexpectedToken {
                    token: vector[start + tsize],
                    point: start + tsize,
                },
            })
        }
        Some(v) => v,
    };
    tsize += size;

    if vector[start + tsize] != b'e' {
        Err(DecodeError {
            reason: DecodeErrorReason::UnexpectedToken {
                token: vector[start + tsize],
                point: start + tsize,
            },
        })
    } else {
        tsize += 1;
        Ok((BencodexValue::Number(number), tsize))
    }
}

fn read_number(s: &[u8]) -> Option<(BigInt, usize)> {
    if s.len() == 0 {
        return None;
    }

    let is_negative = s[0] == b'-';
    if s.len() == 1 && is_negative {
        return None;
    }

    let mut size: usize = is_negative as usize;
    while size < s.len() {
        match s[size] {
            b'0'..=b'9' => {
                size += 1;
                continue;
            }
            _ => break,
        };
    }

    if is_negative && size == 1 || size == 0 {
        None
    } else {
        Some((
            BigInt::from_str(&String::from_utf8(s[..size].to_vec()).unwrap()).unwrap(),
            size,
        ))
    }
}

impl Decode for Vec<u8> {
    fn decode(self) -> Result<BencodexValue, DecodeError> {
        match decode_impl(&self, 0) {
            Ok(v) => Ok(v.0),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    mod decode_impl {
        use super::super::*;

        #[test]
        fn should_return_error_with_overflowed_start() {
            let expected_error = DecodeError {
                reason: DecodeErrorReason::InvalidBencodexValue,
            };
            assert_eq!(expected_error, decode_impl(&vec![], 1).unwrap_err());
            assert_eq!(
                expected_error,
                decode_impl(&vec![b'1', b'2'], 2).unwrap_err()
            );
            assert_eq!(
                expected_error,
                decode_impl(&vec![b'1', b'2'], 20).unwrap_err()
            );
        }

        #[test]
        fn should_return_unexpected_token_error_with_invalid_source() {
            assert_eq!(
                DecodeError {
                    reason: DecodeErrorReason::UnexpectedToken {
                        token: b'x',
                        point: 0,
                    }
                },
                decode_impl(&vec![b'x'], 0).unwrap_err()
            );
            assert_eq!(
                DecodeError {
                    reason: DecodeErrorReason::UnexpectedToken {
                        token: b'k',
                        point: 4,
                    }
                },
                decode_impl(&vec![b'x', b'y', b'z', b'o', b'k'], 4).unwrap_err()
            );
        }
    }

    mod decode_error {
        mod display_impl {
            use super::super::super::*;

            #[test]
            fn fmt() {
                assert_eq!(
                    "DecodeError (reason: InvalidBencodexValue)",
                    DecodeError {
                        reason: DecodeErrorReason::InvalidBencodexValue
                    }
                    .to_string()
                )
            }
        }
    }

    mod read_number {
        use super::super::*;

        #[test]
        fn should_return_none() {
            assert_eq!(None, read_number(b""));
        }

        #[test]
        fn should_return_ok_with_positive() {
            assert_eq!(Some((BigInt::from(1), 1)), read_number(b"1"));
            assert_eq!(Some((BigInt::from(326), 3)), read_number(b"326"));
        }

        #[test]
        fn should_return_ok_with_negative() {
            assert_eq!(Some((BigInt::from(-1), 2)), read_number(b"-1"));
            assert_eq!(Some((BigInt::from(-845), 4)), read_number(b"-845"));
        }

        #[test]
        fn should_return_none_with_single_minus_sign() {
            assert_eq!(None, read_number(b"-"));
        }

        #[test]
        fn should_return_none_with_single_minus_sign_and_invalid_char() {
            assert_eq!(None, read_number(b"-e"));
            assert_eq!(None, read_number(b"-x"));
        }
    }
}
