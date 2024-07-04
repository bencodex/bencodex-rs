# bencodex-rs

[![build](https://github.com/bencodex/bencodex-rs/actions/workflows/build.yaml/badge.svg)](https://github.com/bencodex/bencodex-rs/actions/workflows/build.yaml) [![codecov](https://codecov.io/gh/bencodex/bencodex-rs/branch/main/graph/badge.svg?token=H0FWUZ2ZF2)](https://codecov.io/gh/bencodex/bencodex-rs) [![Docs-rs](https://docs.rs/bencodex-rs/badge.svg)](https://docs.rs/bencodex-rs/latest/)

The [Rust] implementation of [Bencodex].

[Rust]: https://rust-lang.org/
[Bencodex]: https://bencodex.org/

## Bencodex JSON feature.

bencodex-rs implements [Bencodex JSON] feature, encoding and decoding both.

To use Bencodex JSON feature, you should enable `json` feature.

```toml
bencodex-rs = { version = "<VERSION>", features = ["json"] }
```

[Bencodex JSON]: https://github.com/planetarium/bencodex/blob/main/JSON.md

### Encoding to JSON

To encode from Bencodex to JSON, you can use `to_json` function.

```rust
use bencodex::{ BencodexValue, json::to_json };

let json = to_json(&BencodexValue::Null);
println!("{}", json);
```

There are two ways to encode `BencodexValue::Binary` type, `Hex` and `Base64`. You can choose one way with `bencodex::json::BinaryEncoding`. And you can pass it with `bencodex::json::JsonEncodeOptions` to `bencodex::json::to_json_with_options`.

```rust
use bencodex::BencodexValue;
use bencodex::json::{ BinaryEncoding, JsonEncodeOptions, to_json_with_options };

let json = to_json_with_options(&BencodexValue::Null, JsonEncodeOptions {
  binary_encoding: BinaryEncoding::Base64,
});
println!("{}", json);
```

### Decoding from JSON

To decode from JSON to Bencodex, you can use `from_json_string` and `from_json` function.

```rust
// from_json_string
use bencodex::{ BencodexValue, json::from_json_string };

let result = from_json_string("null");
assert!(result.is_ok());
assert_eq!(result.unwrap(), BencodexValue::Null);
```

```rust
// from_json
use serde_json::from_str;
use bencodex::{ BencodexValue, json::from_json };

let json = from_str("null").unwrap();
let result = from_json(&json);
assert!(result.is_ok());
assert_eq!(result.unwrap(), BencodexValue::Null);
```

### CLI Tool


Also, it provides a CLI tool to encode from Bencodex to JSON and to decode from JSON to Bencodex. You can install it with `json-cli` feature like the below line:

```bash
cargo install bencodex-rs --features json-cli
```

You can use like the below:

```bash
# encode
$ echo -n 'n' | bencodex
null
$ echo -n 'i123e' | bencodex
"123"
$ echo -n '1:\x12' | bencodex
"0x12"
$ echo -n '1:\x12' | bencodex --base64
"b64:Eg=="

# decode
$ echo -n '"123"' | bencodex -d
123
$ echo -n 'null' | bencodex -d
n
```

## Building and Testing

You can build and test the code with [cargo] command.

If you want to build:

```
cargo build
```

If you want to test:

```
cargo test --features test
```

[cargo]: https://github.com/rust-lang/cargo/
