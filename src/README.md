# Zero Escape Encoding

[![Documentation](https://docs.rs/zero_escape/badge.svg)](https://docs.rs/zero_escape/latest/zero_escape/)
[![Repository](https://img.shields.io/badge/github-repo-blue.svg)](https://github.com/FssAy/zero_escape)

The `zero_escape` crate provides functionality for encoding and decoding data using the Zero Escape Encoding method.

## Features

- **Encoding**: Convert your data into Zero Escape encoded format. [View Source](https://github.com/FssAy/zero_escape/blob/master/src/core/encoding.rs)
- **Decoding**: Decode Zero Escape encoded data back to its original format. [View Source](https://github.com/FssAy/zero_escape/blob/master/src/core/decoding.rs)

## Usage

Add `zero_escape` to your `Cargo.toml` dependencies:

```toml
[dependencies]
zero_escape = "0.1.0"
```

Using the encoding functions:
```rust
fn main() {
    // Get the data you want to encode
    let any_data: Vec<u8> = get_any_data();
    
    // Encode and decode using ZEE
    let encoded = zero_escape::encode(any_data);
    let decoded = zero_escape::decode(encoded);
}
```

## License

This project is licensed under the GPL-3.0-only license.
