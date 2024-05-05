use {
    super::{super::fields::field_element::FieldElement, errors::ECCErrors},
    anyhow::{bail, Result},
    ibig::{IBig, UBig},
    std::{
        fmt::{self, Display, Formatter},
        ops::{Add, Mul},
    },
};

#[derive(Debug, PartialEq, Eq, Clone)]
/// Point on a curve of equation y^2 = x^3 + ax + b
///
/// Allowing for Optional values for x and y to handle infinity point. One co-ordinate cannot be None,
/// that will result in an error
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self> {
        match (x, y) {
            // Infinity Point
            (None, None) => Ok(Self::inf(a, b)),

            (Some(x), None) => bail!(ECCErrors::InvalidPoint(Some(x), None)),
            (None, Some(y)) => bail!(ECCErrors::InvalidPoint(None, Some(y))),

            (Some(x), Some(y)) => {
                let rhs = y.pow(IBig::from(2));
                let lhs = x.pow(IBig::from(3)) + (&a * &x) + &b;

                if rhs != lhs {
                    bail!(ECCErrors::InvalidPoint(Some(x), Some(y)))
                }

                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                })
            }
        }
    }

    pub fn inf(a: FieldElement, b: FieldElement) -> Self {
        Self {
            x: None,
            y: None,
            a,
            b,
        }
    }

    pub fn is_inf(&self) -> bool {
        self.x.is_none()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.x.is_none() && self.y.is_none() {
            return write!(f, "Point_INF_{}{}", self.a, self.b);
        }

        write!(
            f,
            "Point_{}{}_{}{}",
            self.x.clone().unwrap(),
            self.y.clone().unwrap(),
            self.a,
            self.b
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("{}", ECCErrors::NotSameCurve(Box::new(self), Box::new(rhs)))
        }

        // self is infinity point
        if self.x.is_none() {
            return rhs;
        }

        // rhs is infinity point
        if rhs.x.is_none() {
            return self;
        }

        let x1 = self.x.clone().unwrap();
        let x2 = rhs.x.clone().unwrap();

        // additive inverse
        if x1 == x2 {
            return Self::inf(self.a, self.b);
        }

        let y1 = self.y.clone().unwrap();
        let y2 = rhs.y.clone().unwrap();
        let prime = x1.prime();

        let two = FieldElement::new(UBig::from(2_u8), prime.clone()).unwrap();
        let three = FieldElement::new(UBig::from(3_u8), prime).unwrap();

        // equal points
        let slope = if self == rhs {
            if y1 == y1.zero() {
                return Self::inf(self.a, self.b);
            }
            ((three * x1.pow(IBig::from(2))) + &self.a) / (two * &y1)
        } else {
            (y2 - &y1) / (&x2 - &x1)
        };

        let x3 = slope.pow(IBig::from(2)) - &x1 - x2;
        let y3 = (slope * (x1 - &x3)) - y1;

        Self {
            x: Some(x3),
            y: Some(y3),
            a: self.a,
            b: self.b,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "{}",
                ECCErrors::NotSameCurve(Box::new(self.clone()), Box::new(rhs.clone()))
            )
        }

        // self is infinity point
        if self.x.is_none() {
            return rhs.clone();
        }

        // rhs is infinity point
        if rhs.x.is_none() {
            return self.clone();
        }

        let x1 = self.x.clone().unwrap();
        let x2 = rhs.x.clone().unwrap();

        // additive inverse
        if x1 == x2 {
            return Self::Output::inf(self.a.clone(), self.b.clone());
        }

        let y1 = self.y.clone().unwrap();
        let y2 = rhs.y.clone().unwrap();
        let prime = x1.prime();

        let two = FieldElement::new(UBig::from(2_u8), prime.clone()).unwrap();
        let three = FieldElement::new(UBig::from(3_u8), prime).unwrap();

        // equal points
        let slope = if self == rhs {
            if y1 == y1.zero() {
                return Self::Output::inf(self.a.clone(), self.b.clone());
            }
            ((three * x1.pow(IBig::from(2))) + &self.a) / (two * &y1)
        } else {
            (y2 - &y1) / (&x2 - &x1)
        };

        let x3 = slope.pow(IBig::from(2)) - &x1 - x2;
        let y3 = (slope * (x1 - &x3)) - y1;

        Self::Output {
            x: Some(x3),
            y: Some(y3),
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "{}",
                ECCErrors::NotSameCurve(Box::new(self), Box::new(rhs.clone()))
            )
        }

        // self is infinity point
        if self.x.is_none() {
            return rhs.clone();
        }

        // rhs is infinity point
        if rhs.x.is_none() {
            return self;
        }

        let x1 = self.x.clone().unwrap();
        let x2 = rhs.x.clone().unwrap();

        // additive inverse
        if x1 == x2 {
            return Self::Output::inf(self.a, self.b);
        }

        let y1 = self.y.clone().unwrap();
        let y2 = rhs.y.clone().unwrap();
        let prime = x1.prime();

        let two = FieldElement::new(UBig::from(2_u8), prime.clone()).unwrap();
        let three = FieldElement::new(UBig::from(3_u8), prime).unwrap();

        // equal points
        let slope = if self == *rhs {
            if y1 == y1.zero() {
                return Self::Output::inf(self.a, self.b);
            }
            ((three * x1.pow(IBig::from(2))) + &self.a) / (two * &y1)
        } else {
            (y2 - &y1) / (&x2 - &x1)
        };

        let x3 = slope.pow(IBig::from(2)) - &x1 - x2;
        let y3 = (slope * (x1 - &x3)) - y1;

        Self::Output {
            x: Some(x3),
            y: Some(y3),
            a: self.a,
            b: self.b,
        }
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Self::Output) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "{}",
                ECCErrors::NotSameCurve(Box::new(self.clone()), Box::new(rhs))
            )
        }

        // self is infinity point
        if self.x.is_none() {
            return rhs;
        }

        // rhs is infinity point
        if rhs.x.is_none() {
            return self.clone();
        }

        let x1 = self.x.clone().unwrap();
        let x2 = rhs.x.clone().unwrap();

        // additive inverse
        if x1 == x2 {
            return Self::Output::inf(self.a.clone(), self.b.clone());
        }

        let y1 = self.y.clone().unwrap();
        let y2 = rhs.y.clone().unwrap();
        let prime = x1.prime();

        let two = FieldElement::new(UBig::from(2_u8), prime.clone()).unwrap();
        let three = FieldElement::new(UBig::from(3_u8), prime).unwrap();

        // equal points
        let slope = if *self == rhs {
            if y1 == y1.zero() {
                return Self::Output::inf(self.a.clone(), self.b.clone());
            }
            ((three * x1.pow(IBig::from(2))) + &self.a) / (two * &y1)
        } else {
            (y2 - &y1) / (&x2 - &x1)
        };

        let x3 = slope.pow(IBig::from(2)) - &x1 - x2;
        let y3 = (slope * (x1 - &x3)) - y1;

        Self::Output {
            x: Some(x3),
            y: Some(y3),
            a: self.a.clone(),
            b: self.b.clone(),
        }
    }
}

impl Mul<Point> for UBig {
    type Output = Point;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let mut coefficient = self;
        let mut current = rhs.clone();
        let mut result = Point::inf(rhs.a, rhs.b);

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
        let mut coefficient = self.clone();
        let mut current = rhs.clone();
        let mut result = Point::inf(rhs.a.clone(), rhs.b.clone());

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

impl Mul<&Point> for UBig {
    type Output = Point;

    fn mul(self, rhs: &Self::Output) -> Self::Output {
        let mut coefficient = self;
        let mut current = rhs.clone();
        let mut result = Point::inf(rhs.a.clone(), rhs.b.clone());

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

impl Mul<Point> for &UBig {
    type Output = Point;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let mut coefficient = self.clone();
        let mut current = rhs.clone();
        let mut result = Point::inf(rhs.a, rhs.b);

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

#[cfg(test)]
mod test {
    use ibig::{ubig, UBig};

    use super::*;

    #[test]
    fn create_valid_point() -> Result<()> {
        let prime = UBig::from(223_u8);
        let a = FieldElement::new(ubig!(0), prime.clone())?;
        let b = FieldElement::new(ubig!(7), prime.clone())?;

        let x = FieldElement::new(ubig!(192), prime.clone())?;
        let y = FieldElement::new(ubig!(105), prime)?;

        let p = Point::new(Some(x), Some(y), a, b);

        assert!(p.is_ok());
        Ok(())
    }

    #[test]
    fn create_inf_point() -> Result<()> {
        let prime = UBig::from(223_u8);
        let a = FieldElement::new(ubig!(0), prime.clone())?;
        let b = FieldElement::new(ubig!(7), prime)?;

        let p = Point::new(None, None, a, b)?;

        assert!(p.is_inf());
        Ok(())
    }

    #[test]
    fn create_invalid_point() -> Result<()> {
        let prime = UBig::from(223_u8);
        let a = FieldElement::new(ubig!(0), prime.clone())?;
        let b = FieldElement::new(ubig!(7), prime.clone())?;

        let x = FieldElement::new(ubig!(200), prime.clone())?;
        let y = FieldElement::new(ubig!(119), prime)?;

        let p = Point::new(Some(x), Some(y), a, b);

        assert!(p.is_err());
        Ok(())
    }

    #[test]
    fn equal_points() -> Result<()> {
        let prime = UBig::from(223_u8);
        let a = FieldElement::new(ubig!(0), prime.clone())?;
        let b = FieldElement::new(ubig!(7), prime.clone())?;

        let x = FieldElement::new(ubig!(17), prime.clone())?;
        let y = FieldElement::new(ubig!(56), prime)?;

        let p1 = Point::new(Some(x.clone()), Some(y.clone()), a.clone(), b.clone())?;
        let p2 = Point::new(Some(x), Some(y), a, b)?;

        assert_eq!(p1, p2);
        Ok(())
    }

    #[test]
    fn unequal_points() -> Result<()> {
        let prime = UBig::from(223_u8);
        let a = FieldElement::new(ubig!(0), prime.clone())?;
        let b = FieldElement::new(ubig!(7), prime.clone())?;

        let x = FieldElement::new(ubig!(1), prime.clone())?;
        let y = FieldElement::new(ubig!(193), prime)?;

        let p1 = Point::new(Some(x), Some(y), a.clone(), b.clone())?;
        let p2 = Point::new(None, None, a, b)?;

        assert_ne!(p1, p2);

        Ok(())
    }

    #[test]
    fn add_points() -> Result<()> {
        let prime = UBig::from(223_u8);
        let a = FieldElement::new(ubig!(0), prime.clone())?;
        let b = FieldElement::new(ubig!(7), prime.clone())?;

        let x1 = FieldElement::new(ubig!(192), prime.clone())?;
        let y1 = FieldElement::new(ubig!(105), prime.clone())?;

        let x2 = FieldElement::new(ubig!(17), prime.clone())?;
        let y2 = FieldElement::new(ubig!(56), prime.clone())?;

        let x3 = FieldElement::new(ubig!(170), prime.clone())?;
        let y3 = FieldElement::new(ubig!(142), prime)?;

        let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone())?;
        let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone())?;
        let p3 = Point::new(Some(x3), Some(y3), a, b)?;

        assert_eq!(p1 + p2, p3);
        Ok(())
    }
}
