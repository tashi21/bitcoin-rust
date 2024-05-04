use {super::field_element::FieldElement, ibig::UBig, thiserror::Error};

#[derive(Debug, PartialEq, Eq, Error)]
/// Errors related to Fields
pub enum FieldErrors {
    #[error("`{0}` not in field range 0 to `{1}`")]
    NotInRange(UBig, UBig),
    #[error("`{0}` not in same field as `{1}`")]
    NotSameField(FieldElement, FieldElement),
}
