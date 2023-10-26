mod macros;

use macros::*;
use crate::*;

#[test]
fn encode_without_escape_byte() {
    let input = b"NO_ESCAPE_BYTE";
    let output = encode(input);
    assert_eq!(&input[..], &output, "output differs from the output, but no escape byte was present");
}

#[test]
fn encode_with_flag_byte() {
    let input = bytes!['Z', 'E', 'E', BYTE_FLAG];
    let output = encode(&input);
    assert_ne!(input, output, "output is the same as input, but flag byte was present");
    assert_eq!(output.len(), input.len()+1, "output length should be higher than input length by 1");

    let trailing = &output[output.len()-2..];
    assert_eq!(trailing, &[BYTE_FLAG, BYTE_REPR_FLAG], "output doesn't end with the proper encoding");
}

#[test]
fn encode_with_escape_byte() {
    let input = bytes!['Z', 'E', 'E', BYTE_ESCAPE];
    let output = encode(&input);
    assert_ne!(input, output, "output is the same as input, but escape byte was present");
    assert_eq!(output.len(), input.len()+1, "output length should be higher than input length by 1");

    let trailing = &output[output.len()-2..];
    assert_eq!(trailing, &[BYTE_FLAG, BYTE_REPR_ESCAPE], "output doesn't end with the proper encoding");
}

#[test]
fn decode_without_escape_byte() {
    let encoded = b"NO_ESCAPE_BYTE";
    let decoded = decode(&encoded).unwrap();
    assert_eq!(&encoded[..], &decoded, "decoded output differs from the input, but no escape byte was present");
}

#[test]
fn decode_with_flag_byte() {
    let encoded = bytes!('Z', 'E', 'E', BYTE_FLAG, BYTE_REPR_FLAG);
    let decoded = decode(&encoded).unwrap();
    assert_ne!(encoded, decoded, "decoded output is the same as input even though the flag byte was present");
}

#[test]
fn decode_with_escape_byte() {
    let encoded = bytes!['Z', 'E', 'E', BYTE_FLAG, BYTE_REPR_ESCAPE];
    let decoded = decode(&encoded).unwrap();
    assert_ne!(encoded, decoded, "decoded output is the same as input even though the escape byte was present");
}
