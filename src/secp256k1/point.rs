use {
    super::{
        constants::{B, SECP256K1_ORDER_RING},
        element::Element,
        errors::SECP256K1CurveError,
    },
    anyhow::{bail, Result},
    ibig::{modular::IntoModulo, IBig, UBig},
    std::{
        fmt::{self, Display, Formatter},
        ops::{Add, Mul},
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Point on the SECP256K1 Curve
///
/// Allowing for Optional values for x and y to handle infinity point. Only one coordinate cannot be None,
/// that will result in an error
pub struct Point {
    x: Option<Element>,
    y: Option<Element>,
}

impl Point {
    /// A point on the SECP256K1 Curve
    pub fn new(x: Option<Element>, y: Option<Element>) -> Result<Self> {
        match (x, y) {
            // Infinity Point
            (None, None) => Ok(Self::inf()),

            (Some(x), None) => bail!(SECP256K1CurveError::InvalidPoint(Some(x), None)),
            (None, Some(y)) => bail!(SECP256K1CurveError::InvalidPoint(None, Some(y))),

            (Some(x), Some(y)) => {
                let rhs = y.pow(IBig::from(2));
                // xbu^3 + B
                let lhs = B.with(|b| x.pow(IBig::from(3)) + b);

                if rhs != lhs {
                    bail!(SECP256K1CurveError::InvalidPoint(Some(x), Some(y)))
                }

                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                })
            }
        }
    }

    /// Infnity point of the SECP256K1 Curve
    pub fn inf() -> Self {
        Self { x: None, y: None }
    }

    // If a point is the infinity point
    pub fn is_inf(&self) -> bool {
        self.x.is_none()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.x.is_none() {
            return write!(f, "SECP256K1_Curve_INF");
        }

        write!(
            f,
            "SECP256K1_Curve_{}{}",
            self.x.as_ref().unwrap(),
            self.y.as_ref().unwrap(),
        )
    }
}

impl Add for Point {
    type Output = Self;

    /// Add two points on the SECP256K1 Curve
    fn add(self, rhs: Self) -> Self::Output {
        // self is infinity point
        if self.x.is_none() {
            return rhs;
        }

        // rhs is infinity point
        if rhs.x.is_none() {
            return self;
        }
        // x coordinates are not none, hence y coordinates are also not none
        let x1 = self.x.as_ref().unwrap();
        let x2 = rhs.x.as_ref().unwrap();
        let y1 = self.y.as_ref().unwrap();
        let y2 = rhs.y.as_ref().unwrap();

        // additive inverse
        if x1 == x2 && y1 != y2 {
            return Self::inf();
        }

        let slope = if self == rhs {
            // points are same
            // y coordinates are zero
            if y1.is_zero() {
                return Self::inf();
            }
            (Element::new(UBig::from(3_u8)).unwrap() * x1.pow(IBig::from(2)))
                / (Element::new(UBig::from(2_u8)).unwrap() * y1)
        } else {
            // points have no same coordinates
            (y2 - y1) / (x2 - x1)
        };

        let x3 = slope.pow(IBig::from(2)) - x1 - x2;
        let y3 = slope * (x1 - &x3) - y1;

        Self {
            x: Some(x3),
            y: Some(y3),
        }
    }
}

impl Add for &Point {
    type Output = Point;

    /// Add two points on the SECP256K1 Curve
    fn add(self, rhs: Self) -> Self::Output {
        Point::add(self.clone(), rhs.clone())
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Point::add(self, rhs.clone())
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Point::add(self.clone(), rhs)
    }
}

impl Mul<Point> for UBig {
    type Output = Point;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        // we know order of SECP256K1 curve so we can mod the coefficient to optimise calculation
        let mut coefficient = SECP256K1_ORDER_RING.with(|r| self.into_modulo(r).residue());
        let mut current = rhs;
        let mut result = Point::inf();

        let zero = UBig::from(0_u8);
        let one = UBig::from(1_u8);

        loop {
            if coefficient == zero {
                // return infinity if coefficient is 0 in first iteration
                break result;
            };

            if (&coefficient & 1) == one {
                result = &result + &current; // update during odd iterations
            }

            current = &current + &current; // adjust for even iteration
            coefficient >>= 1; // cut down two iterations
        }
    }
}

impl Mul<&Point> for &UBig {
    type Output = Point;

    fn mul(self, rhs: &Self::Output) -> Self::Output {
        UBig::mul(self.clone(), rhs.clone())
    }
}

impl Mul<&Point> for UBig {
    type Output = Point;

    fn mul(self, rhs: &Self::Output) -> Self::Output {
        UBig::mul(self, rhs.clone())
    }
}

impl Mul<Point> for &UBig {
    type Output = Point;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        UBig::mul(self.clone(), rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::secp256k1::constants::{SECP256K1_GENERATOR_POINT, SECP256K1_ORDER};

    use super::*;

    #[test]
    fn generator_on_curve() -> Result<()> {
        let _ = SECP256K1_GENERATOR_POINT.with(|g| g.clone());
        Ok(())
    }

    #[test]
    fn order_n_test() -> Result<()> {
        let g = SECP256K1_GENERATOR_POINT.with(|g| g.clone());
        let n = SECP256K1_ORDER.with(|n| n.clone());
        let inf = n * g;
        assert!(inf.is_inf());
        Ok(())
    }
}
