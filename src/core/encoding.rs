use super::*;

pub fn encode(input: impl AsRef<[u8]>) -> Vec<u8> {
    let (input_iter, mut output) = prep(input.as_ref());

    for byte in input_iter {
        match byte {
            BYTE_ESCAPE => output.extend_from_slice(&[BYTE_FLAG, BYTE_REPR_ESCAPE]),
            BYTE_FLAG => output.extend_from_slice(&[BYTE_FLAG, BYTE_REPR_FLAG]),
            byte => output.push(byte),
        }
    }

    output
}
