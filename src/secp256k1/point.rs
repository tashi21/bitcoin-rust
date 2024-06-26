use {
    super::{
        constants::{B, G, N_RING, P},
        element::Element,
        errors::SECP256K1CurveError,
        signature::Signature,
    },
    anyhow::{bail, Result},
    hex::encode_upper,
    ibig::{modular::IntoModulo, UBig},
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
                // y^2
                let rhs = y.pow("2", 10)?;
                // x^3 + B
                let lhs = B.with(|b| b + x.pow("3", 10).unwrap());

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

    /// Infinity point of the SECP256K1 Curve
    pub fn inf() -> Self {
        Self { x: None, y: None }
    }

    // If a point is the infinity point
    pub fn is_inf(&self) -> bool {
        self.x.is_none()
    }

    /// Verify the signature generated by public key `self`
    ///
    /// `z` is the message hash (256 bits)
    pub fn verify(&self, z: UBig, signature: Signature) -> bool {
        let u = N_RING.with(|o| (z.into_modulo(o) / signature.s().into_modulo(o)).residue());

        let v = N_RING
            .with(|o| (signature.r().into_modulo(o) / signature.s().into_modulo(o)).residue());

        let ug = G.with(|g| u * g);
        // self = P = eG => vP = veG
        let vp = v * self;

        let r = ug + vp;

        r.x.unwrap().num() == signature.r()
    }

    pub fn x(&self) -> UBig {
        self.x.clone().unwrap().num()
    }

    pub fn y(&self) -> UBig {
        self.y.clone().unwrap().num()
    }

    /// Return the SEC serialisation of the point
    pub fn serialise(&self, compressed: bool) -> String {
        if compressed {
            let mut prefix = b"\x03"; // assume y is odd
            if self.y().trailing_zeros().is_none() || self.y().trailing_zeros().unwrap() > 0 {
                // y is even
                prefix = b"\x02"; // update prefix
            }
            let enc = [prefix, self.x().to_be_bytes().as_slice()].concat();
            encode_upper(enc)
        } else {
            let enc = [
                b"\x04",
                self.x().to_be_bytes().as_slice(),
                &self.y().to_be_bytes().as_slice(),
            ]
            .concat();
            encode_upper(enc)
        }
    }

    /// Parse the hex encoded SEC serialisation of the point
    pub fn parse(self, sec_hex: &str) -> Result<Self> {
        // get prefix of point
        let prefix = sec_hex.as_bytes()[0];

        // uncompressed SEC serialisation
        if prefix == b'\x04' {
            return Ok(Self {
                x: Some(Element::new(sec_hex.get(1..33).unwrap(), 16)?),
                y: Some(Element::new(sec_hex.get(33..65).unwrap(), 16)?),
            });
        }

        // compressed SEC serialisation
        let x = Element::new(sec_hex.get(1..33).unwrap(), 16)?;
        let beta = B.with(|b| b + x.pow("3", 10).unwrap()).sqrt();

        let odd_y;
        let even_y;

        // if beta is even
        if beta.num() % 2 == 0 {
            odd_y = Element::new(&P.with(|p| p - beta.num()).to_string(), 10)?;
            even_y = beta;
        } else {
            // beta is odd
            even_y = Element::new(&P.with(|p| p - beta.num()).to_string(), 10)?;
            odd_y = beta;
        }

        // y is even
        if prefix == b'\x02' {
            Ok(Self::new(Some(x), Some(even_y))?)
        } else {
            // y is odd
            Ok(Self::new(Some(x), Some(odd_y))?)
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.x.is_none() {
            return write!(f, "INF");
        }

        write!(
            f,
            "(x: {}, y: {})",
            self.x.as_ref().unwrap(),
            self.y.as_ref().unwrap(),
        )
    }
}

impl Add for Point {
    type Output = Self;

    /// Add two points on the SECP256K1 Curve
    fn add(self, rhs: Self) -> Self::Output {
        match (&self.x, &self.y, &rhs.x, &rhs.y) {
            // self is infinity point
            (None, _, _, _) => rhs,

            // rhs is infinity point
            (_, _, None, _) => self,

            // both points have some values
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                // adding additive inverses
                if x1 == x2 && y1 != y2 {
                    return Self::inf();
                }

                // points are same
                let slope = if x1 == x2 {
                    // line will be a tangent if y co-ordinates are 0
                    if y1.is_zero() {
                        return Self::inf();
                    }

                    // y co-ordinates are not zero
                    let two = Element::new("2", 10).unwrap();
                    let three = Element::new("3", 10).unwrap();
                    // slope for when points are the same
                    (three * x1.pow("2", 10).unwrap()) / (&two * y1)
                } else {
                    // slope for when points are different
                    (y2 - y1) / (x2 - x1)
                };

                let x3 = slope.pow("2", 10).unwrap() - x1 - x2;
                let y3 = slope * (x1 - &x3) - y1;

                Self::new(Some(x3), Some(y3)).unwrap()
            }
            _ => Self::inf(),
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
        let mut coefficient = N_RING.with(|r| self.into_modulo(r).residue());
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
    use {
        super::*,
        crate::secp256k1::constants::{G, N},
        ibig::ubig,
    };

    #[test]
    fn g_on_curve() -> Result<()> {
        // call global variable and initialise it
        // if it does then point coordinates are valid
        G.with(|g| g.clone());
        Ok(())
    }

    #[test]
    fn order_of_g_is_n() -> Result<()> {
        let g = G.with(|g| g.clone());
        let n = N.with(|n| n.clone());
        // order * generator will always be infinity
        let p = n * g;

        assert!(p.is_inf());
        Ok(())
    }

    #[test]
    fn scalar_mult_and_add() -> Result<()> {
        let g = G.with(|g| g.clone());
        let scalar = ubig!(3);

        assert_eq!(scalar * &g, &g + &g + g);
        Ok(())
    }
}
