use {
    super::{super::fields::field_element::FieldElement, point::Point},
    thiserror::Error,
};

#[derive(Debug, PartialEq, Eq, Error)]
/// Errors related to Fields
pub enum ECCErrors {
    #[error("(`{0:?}`, {1:?}`) not on the curve")]
    InvalidPoint(Option<FieldElement>, Option<FieldElement>),
    #[error("`{0}` not in same curve as `{1}`")]
    NotSameCurve(Point, Point),
}
