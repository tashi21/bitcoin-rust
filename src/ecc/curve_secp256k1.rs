use ibig::{modular::ModuloRing, UBig};

pub struct SECP256K1Field {
    /// Number in the field
    num: UBig,
    /// Prime of the field
    prime: UBig,
    /// Ring of the Field
    ring: ModuloRing,
}
