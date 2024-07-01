use bencodex::json::encode::{to_json_with_options, BinaryEncoding, JsonOptions};
use bencodex::Decode;
use clap::Parser;
use std::io::Read;

/// A program to encode and decode between Bencodex and JSON.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Encode Bencodex Binary by base64 string.
    /// If not given, it will encode as hexadecimal string.
    #[arg(short, long)]
    base64: bool,
}

fn main() {
    let args = Args::parse();
    let mut buf = Vec::new();
    if let Err(err) = std::io::stdin().read_to_end(&mut buf) {
        eprintln!("{:?}", err);
        return;
    }

    let decoded = match buf.decode() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to decode. {:?}", err);
            return;
        }
    };

    let json_encode_options = JsonOptions {
        binary_encoding: if args.base64 {
            BinaryEncoding::Base64
        } else {
            BinaryEncoding::Hex
        },
    };

    println!("{}", to_json_with_options(&decoded, json_encode_options));
}
