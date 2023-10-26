mod encoding;
mod decoding;

use std::iter::Map;
use std::slice::Iter;
pub use encoding::*;
pub use decoding::*;

pub(crate) const BYTE_ESCAPE: u8 = 0x00;
pub(crate) const BYTE_FLAG: u8 = 0xFF;
pub(crate) const BYTE_REPR_ESCAPE: u8 = 0x01;
pub(crate) const BYTE_REPR_FLAG: u8 = 0x02;

type InputIter<'a> = Map<Iter<'a, u8>, fn(&'a u8) -> u8>;

fn prep(input: &[u8]) -> (InputIter, Vec<u8>) {
    let output = Vec::with_capacity(input.len());
    let input_iter: InputIter = input.iter().map(|byte_ref| *byte_ref);
    (input_iter, output)
}
