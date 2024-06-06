use {
    super::{element::Element, point::Point},
    ibig::{modular::ModuloRing, UBig},
};

thread_local! {
    /// `b` coefficient of the SECP256K1 Field
    ///
    /// Hex representation: 00000007
    pub static B: Element = Element::new(UBig::from(7_u8)).unwrap()
}

thread_local! {
    /// Order of the SECP256K1 Field
    ///
    /// Hex representation: FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6
    /// AF48A03B BFD25E8C D0364141
    pub static N: UBig = UBig::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap()
}

thread_local! {
    /// Prime of the SECP256K1 Field
    ///
    /// Hex representation: FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF
    /// FFFFFFFF FFFFFFFE FFFFFC2F
    pub static P: UBig = UBig::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap()
}

thread_local! {
    /// Generator Point of the SECP256K1 Curve
    ///
    /// Hex representation of x: 79BE667E F9DCBBAC 55A06295 CE870B07
    /// 029BFCDB 2DCE28D9 59F2815B 16F81798
    ///
    /// Hex representation of y: 483ADA77 26A3C465 5DA4FBFC 0E1108A8
    /// FD17B448 A6855419 9C47D08F FB10D4B8
    pub static G: Point = Point::new(
        Some(
            Element::new(
                UBig::from_str_radix(
                    "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
                    16)
                .unwrap())
            .unwrap()),
        Some(
            Element::new(
                UBig::from_str_radix(
                    "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
                    16)
                .unwrap())
            .unwrap()))
    .unwrap()
}

thread_local! {
    /// Order ring of the SECP256K1 Field
    pub static N_RING: ModuloRing = ModuloRing::new(
        &UBig::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap()
    )
}

thread_local! {
    /// Prime ring of the SECP256K1 Field
    pub static P_RING: ModuloRing = ModuloRing::new(
        &UBig::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap()
    )
}
