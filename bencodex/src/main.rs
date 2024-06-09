use bencodex::Decode;
use std::io::Read;

fn main() {
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

    println!("{}", decoded);
}
