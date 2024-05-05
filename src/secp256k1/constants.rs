use {
    super::{element::Element, point::Point},
    ibig::{modular::ModuloRing, UBig},
};

thread_local! {
    /// `a` coefficient of the SECP256K1 Field
    pub static A: Element = Element::new(UBig::from(0_u8)).unwrap()
}

thread_local! {
    /// `b` coefficient of the SECP256K1 Field
    pub static B: Element = Element::new(UBig::from(7_u8)).unwrap()
}

thread_local! {
    /// Order of the SECP256K1 Field
    pub static SECP256K1_ORDER: UBig = UBig::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap()
}

thread_local! {
    /// Prime of the SECP256K1 Field
    pub static SECP256K1_PRIME: UBig = UBig::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap()
}

thread_local! {
    /// Generator Point of the SECP256K1 Curve
    pub static SECP256K1_GENERATOR_POINT: Point = Point::new(
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
    pub static SECP256K1_ORDER_RING: ModuloRing = ModuloRing::new(
        &UBig::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap()
    )
}

thread_local! {
    /// Prime ring of the SECP256K1 Field
    pub static SECP256K1_RING: ModuloRing = ModuloRing::new(
        &UBig::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap()
    )
}
