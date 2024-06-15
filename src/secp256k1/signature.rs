use {
    hex::encode_upper,
    ibig::UBig,
    std::fmt::{self, Display, Formatter},
};

pub struct Signature {
    r: UBig,
    s: UBig,
}

impl Signature {
    /// Create a new signature from the given r and s values.
    pub fn new(r: UBig, s: UBig) -> Self {
        Self { r, s }
    }

    /// Return the r value of the signature.
    pub fn r(&self) -> UBig {
        self.r.clone()
    }

    /// Return the s value of the signature.
    pub fn s(&self) -> UBig {
        self.s.clone()
    }

    /// Serialises the signature using DER encoding.
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

        encode_upper(enc)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:X}, {:X})", self.r, self.s)
    }
}
