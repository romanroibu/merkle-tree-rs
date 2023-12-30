use std::fmt::{Debug, Display};

use merkle_tree::NodeHash;

#[derive(Clone, PartialEq)]
pub(crate) struct HexValue {
    data: [u8; 32],
}

impl HexValue {
    #[allow(dead_code)]
    pub(crate) fn new(data: [u8; 32]) -> Self {
        HexValue { data }
    }
}

impl Debug for HexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.data))
    }
}

impl Display for HexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.data))
    }
}

impl NodeHash for HexValue {}

impl AsRef<[u8]> for HexValue {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl From<[u8; 32]> for HexValue {
    fn from(value: [u8; 32]) -> Self {
        HexValue { data: value }
    }
}

impl Into<[u8; 32]> for HexValue {
    fn into(self) -> [u8; 32] {
        self.data
    }
}

// See: https://github.com/Kixunil/hex_lit/blob/master/src/lib.rs
#[macro_export]
macro_rules! hex {
    ($hex:expr) => {{
        let decode_digit = |digit: u8| match digit {
            b'0'..=b'9' => digit - b'0',
            b'a'..=b'f' => digit - b'a' + 10,
            b'A'..=b'F' => digit - b'A' + 10,
            _ => panic!("invalid digit"),
        };

        const HEX: &str = $hex;
        const _HEX_LENGTH_MUST_BE_64: () = [()][$hex.len() - 64];

        let hex_bytes = HEX.as_bytes();
        let mut data = [0u8; 32];
        let mut pos = 0;
        loop {
            if pos >= data.len() {
                break;
            }

            // Decode byte
            let c1 = decode_digit(HEX.as_bytes()[pos * 2]);
            let c2 = decode_digit(hex_bytes[pos * 2 + 1]);
            data[pos] = c1 << 4 | c2;
            pos += 1
        }
        HexValue::new(data)
    }};
}
