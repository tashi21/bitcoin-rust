use std::fmt::{self, Display, Formatter};

use ibig::UBig;

pub struct Signature {
    r: UBig,
    s: UBig,
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(r: {:X}, s: {:X})", self.r, self.s)
    }
}
