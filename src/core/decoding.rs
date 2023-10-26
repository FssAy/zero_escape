use crate::Error;
use super::*;

pub fn decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
    let (input_iter, mut output) = prep(input.as_ref());

    let mut flag_detected = false;
    for (pos, mut byte) in input_iter.enumerate() {
        if flag_detected {
            byte = match byte {
                BYTE_REPR_ESCAPE => BYTE_ESCAPE,
                BYTE_REPR_FLAG => BYTE_FLAG,
                _ => return Err(
                    Error::new(pos, byte, Some(BYTE_FLAG))
                ),
            };

            output.push(byte);
            flag_detected = false;
            continue;
        }

        match byte {
            BYTE_FLAG => flag_detected = true,
            BYTE_ESCAPE => return Err(Error::new(pos, byte, None)),
            byte => output.push(byte),
        }
    }

    Ok(output)
}

// ToDo: Make more optimal decoding function.
