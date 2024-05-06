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
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(r: {:X}, s: {:X})", self.r, self.s)
    }
}
