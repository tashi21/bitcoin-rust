use std::fmt::{self, Display, Formatter};

use ibig::UBig;

pub struct Signature {
    r: UBig,
    s: UBig,
}

impl Signature {
    pub fn new(r: UBig, s: UBig) -> Self {
        Self { r, s }
    }

    pub fn r(&self) -> UBig {
        self.r.clone()
    }

    pub fn s(&self) -> UBig {
        self.s.clone()
    }

    pub fn serialise(&self) -> String {
        // encode r value
        let mut r_bytes = self.r.to_be_bytes();
        // first byte >= 0x80
        if r_bytes[0] & 0x80 != 0 {
            r_bytes.insert(0, 0x00);
        }
        let mut enc = [&[2_u8, r_bytes.len() as u8], r_bytes.as_slice()].concat();

        // encode s value
        let mut s_bytes = self.s.to_be_bytes();
        // first byte >= 0x80
        if s_bytes[0] & 0x80 != 0 {
            s_bytes.insert(0, 0x00);
        }
        enc = [
            enc.as_slice(),
            &[2_u8, s_bytes.len() as u8],
            s_bytes.as_slice(),
        ]
        .concat();

        // create final signature
        enc = [&[0x30_u8, enc.len() as u8], enc.as_slice()].concat();

        format!("{:X?}", enc)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:X}, {:X})", self.r, self.s)
    }
}
