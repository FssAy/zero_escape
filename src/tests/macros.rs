macro_rules! bytes {
    ( $($value:expr),* ) => {
        {
            vec![
                $(
                    $value as u8,
                )*
            ]
        }
    };
}

pub(super) use bytes;
