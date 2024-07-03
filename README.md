# bencodex-rs

[![build](https://github.com/bencodex/bencodex-rs/actions/workflows/build.yaml/badge.svg)](https://github.com/bencodex/bencodex-rs/actions/workflows/build.yaml) [![codecov](https://codecov.io/gh/bencodex/bencodex-rs/branch/main/graph/badge.svg?token=H0FWUZ2ZF2)](https://codecov.io/gh/bencodex/bencodex-rs) [![Docs-rs](https://docs.rs/bencodex-rs/badge.svg)](https://docs.rs/bencodex-rs/latest/)

The [Rust] implementation of [Bencodex].

[Rust]: https://rust-lang.org/
[Bencodex]: https://bencodex.org/

## Encoding to JSON

bencodex-rs implement Bencodex JSON feature. You can use `to_json` function to encode from Bencodex to JSON.

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

## Building and Testing

You can build and test the code with [cargo] command.

If you want to build:

```
cargo build
```

If you want to test:

```
cargo test
```

[cargo]: https://github.com/rust-lang/cargo/
