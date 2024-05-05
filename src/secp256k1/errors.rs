use {super::element::Element, ibig::UBig, thiserror::Error};

#[derive(Debug, PartialEq, Eq, Error)]
/// Errors related to Fields
pub enum SECP256K1CurveError {
    #[error("(`{0:?}`, {1:?}`) not on the curve")]
    InvalidPoint(Option<Element>, Option<Element>),
}

#[derive(Debug, PartialEq, Eq, Error)]
/// Errors related to Fields
pub enum SECP256K1FieldError {
    #[error("`{0}` not in field range 0 to 2^256 - 2^32 - 977")]
    NotInRange(UBig),
}
