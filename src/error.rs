use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error {
    InvalidRepr {
        pos: usize,
        flag: u8,
        byte: u8,
    },
    UnencodedInput {
        pos: usize,
        byte: u8,
    },
}

impl Error {
    pub(crate) fn new(pos: usize, byte: u8, flag: Option<u8>) -> Self {
        if let Some(flag) = flag {
            Self::InvalidRepr { pos, flag, byte }
        } else {
            Self::UnencodedInput { pos, byte }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::InvalidRepr {
                pos, flag, byte
            } => format!(
                "Detected a flag [0x{:X}], \
                but the following byte [0x{:X}] \
                at position [{}] \
                is not a valid representation.",
                flag,
                byte,
                pos,
            ),
            Error::UnencodedInput {
                pos, byte
            } => format!(
                "Provided input that was not encoded and \
                contains illegal byte [0x{:X}] \
                at position [{}].",
                byte,
                pos,
            ),
        } )
    }
}

impl std::error::Error for Error {}
