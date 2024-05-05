use {
    super::{
        constants::{SECP256K1_PRIME, SECP256K1_RING},
        errors::SECP256K1FieldError,
    },
    anyhow::{bail, Result},
    ibig::{modular::IntoModulo, IBig, UBig},
    std::{
        fmt::{self, Display, Formatter},
        ops::{Add, Div, Mul, Sub},
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// An element belonging to the SECP256K1 Field
pub struct Element {
    /// Number in the field
    num: UBig,
}

impl Element {
    /// Create a new Field Element with the given prime as the field size
    pub fn new(num: UBig) -> Result<Self> {
        if SECP256K1_PRIME.with(|p| num >= *p) {
            bail!(SECP256K1FieldError::NotInRange(num));
        }

        Ok(Self { num })
    }

    /// Raises self to the power of `exp` and returns the new computed value
    pub fn pow(&self, exp: IBig) -> Self {
        Self {
            num: SECP256K1_RING.with(|r| (&self.num).into_modulo(r).pow_signed(&exp).residue()),
        }
    }

    /// Check if an element is 0
    pub fn is_zero(&self) -> bool {
        self.num == UBig::from(0_u8)
    }

    /// Return the Field Element number
    pub fn num(&self) -> UBig {
        self.num.clone()
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SECP256K1_Field_{}", self.num)
    }
}

impl Add for Element {
    type Output = Self;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) + rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Add for &Element {
    type Output = Element;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) + (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Add<&Element> for Element {
    type Output = Self;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) + (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Add<Element> for &Element {
    type Output = Element;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) + rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Sub for Element {
    type Output = Self;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) - rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Sub for &Element {
    type Output = Element;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) - (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Sub<&Element> for Element {
    type Output = Self;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) - (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Sub<Element> for &Element {
    type Output = Element;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) - rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Mul for Element {
    type Output = Self;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) * rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Mul for &Element {
    type Output = Element;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) * (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Mul<&Element> for Element {
    type Output = Self;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) * (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Mul<Element> for &Element {
    type Output = Element;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) * rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Div for Element {
    type Output = Self;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) / rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Div for &Element {
    type Output = Element;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) / (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Div<&Element> for Element {
    type Output = Self;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| (self.num.into_modulo(r) / (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Div<Element> for &Element {
    type Output = Element;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: SECP256K1_RING
                .with(|r| ((&self.num).into_modulo(r) / rhs.num.into_modulo(r)).residue()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ibig::{ibig, ubig};

    #[test]
    fn create_valid_field_elem() -> Result<()> {
        let e = Element::new(ubig!(1));

        assert!(e.is_ok());
        Ok(())
    }

    #[test]
    fn create_invalid_field_elem() -> Result<()> {
        let e = Element::new(SECP256K1_PRIME.with(|p| p + 1));

        assert!(e.is_err());
        Ok(())
    }

    #[test]
    fn equal_field_elems() -> Result<()> {
        let e1 = Element::new(ubig!(2))?;
        let e2 = Element::new(ubig!(2))?;

        assert_eq!(e1, e2);
        Ok(())
    }

    #[test]
    fn unequal_field_elems() -> Result<()> {
        let e1 = Element::new(ubig!(2))?;
        let e2 = Element::new(ubig!(3))?;

        assert_ne!(e1, e2);
        Ok(())
    }

    #[test]
    fn is_zero() -> Result<()> {
        let e = Element::new(ubig!(0))?;

        assert!(e.is_zero());
        Ok(())
    }

    #[test]
    fn get_num() -> Result<()> {
        let e = Element::new(ubig!(3))?;

        assert_eq!(e.num(), ubig!(3));
        Ok(())
    }

    #[test]
    fn add_elems() -> Result<()> {
        let e1 = Element::new(ubig!(7))?;
        let e2 = Element::new(ubig!(12))?;
        let e3 = Element::new(ubig!(19))?;

        assert_eq!(e1 + e2, e3);
        Ok(())
    }

    #[test]
    fn sub_elems() -> Result<()> {
        let e1 = Element::new(ubig!(7))?;
        let e2 = Element::new(ubig!(12))?;
        let e3 = Element::new(
            UBig::from_str_radix(
                "115792089237316195423570985008687907853269984665640564039457584007908834671658",
                10,
            )
            .unwrap(),
        )?;

        assert_eq!(e1 - e2, e3);
        Ok(())
    }

    #[test]
    fn mul_elems() -> Result<()> {
        let e1 = Element::new(ubig!(3))?;
        let e2 = Element::new(ubig!(12))?;
        let e3 = Element::new(ubig!(36))?;

        assert_eq!(e1 * e2, e3);
        Ok(())
    }

    #[test]
    fn div_elems() -> Result<()> {
        let e1 = Element::new(ubig!(2))?;
        let e2 = Element::new(ubig!(7))?;
        let e3 = Element::new(
            UBig::from_str_radix(
                "82708635169511568159693560720491362752335703332600402885326845719934881908331",
                10,
            )
            .unwrap(),
        )?;

        assert_eq!(e1 / e2, e3);
        Ok(())
    }

    #[test]
    fn exp_elem() -> Result<()> {
        let e1 = Element::new(ubig!(7))?;
        let e2 = Element::new(
            UBig::from_str_radix(
                "91823464351457740977292442922341431300552291046805345244117967493152195424759",
                10,
            )
            .unwrap(),
        )?;

        assert_eq!(e1.pow(ibig!(-3)), e2);
        Ok(())
    }
}
