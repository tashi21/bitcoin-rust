use {
    super::{
        constants::{P, P_RING},
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
    /// Create a new Field Element
    ///
    /// `num` is the number in the field
    ///
    /// `radix` is the radix of the number, between 2 and 36 (inclusive)
    pub fn new(num: &str, radix: u32) -> Result<Self> {
        let num = UBig::from_str_radix(num, radix)?;

        // check if given number is in field range
        if P.with(|p| num >= *p) {
            bail!(SECP256K1FieldError::NotInRange(num));
        }

        Ok(Self { num })
    }

    /// Returns the value of the field element raised to the power of `exp`
    pub fn pow(&self, exp: &str, radix: u32) -> Result<Self> {
        let exp = IBig::from_str_radix(exp, radix)?;

        Ok(Self {
            num: P_RING.with(|r| (&self.num).into_modulo(r).pow_signed(&exp).residue()),
        })
    }

    /// Returns the square root of the field element
    pub fn sqrt(&self) -> Self {
        // P % 4 == 3
        // so (P + 1) / 4 is an integer
        self.pow(&P.with(|p| IBig::from((p + 1) / 4)).to_string(), 10)
            .unwrap()
    }

    /// Check if an element is 0
    pub fn is_zero(&self) -> bool {
        self.num.bit_len() == 0
    }

    /// Return the Field Element number
    pub fn num(&self) -> UBig {
        self.num.clone()
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.num)
    }
}

impl Add for Element {
    type Output = Self;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) + rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Add for &Element {
    type Output = Element;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING
                .with(|r| ((&self.num).into_modulo(r) + (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Add<&Element> for Element {
    type Output = Self;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) + (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Add<Element> for &Element {
    type Output = Element;

    /// Add two elements of the SECP256K1 Field
    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| ((&self.num).into_modulo(r) + rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Sub for Element {
    type Output = Self;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) - rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Sub for &Element {
    type Output = Element;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING
                .with(|r| ((&self.num).into_modulo(r) - (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Sub<&Element> for Element {
    type Output = Self;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) - (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Sub<Element> for &Element {
    type Output = Element;

    /// Subtract two elements of the SECP256K1 Field
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| ((&self.num).into_modulo(r) - rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Mul for Element {
    type Output = Self;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) * rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Mul for &Element {
    type Output = Element;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING
                .with(|r| ((&self.num).into_modulo(r) * (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Mul<&Element> for Element {
    type Output = Self;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) * (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Mul<Element> for &Element {
    type Output = Element;

    /// Multiply two elements of the SECP256K1 Field
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| ((&self.num).into_modulo(r) * rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Div for Element {
    type Output = Self;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) / rhs.num.into_modulo(r)).residue()),
        }
    }
}

impl Div for &Element {
    type Output = Element;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            num: P_RING
                .with(|r| ((&self.num).into_modulo(r) / (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Div<&Element> for Element {
    type Output = Self;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: &Self) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| (self.num.into_modulo(r) / (&rhs.num).into_modulo(r)).residue()),
        }
    }
}

impl Div<Element> for &Element {
    type Output = Element;

    /// Divide two elements of the SECP256K1 Field
    fn div(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            num: P_RING.with(|r| ((&self.num).into_modulo(r) / rhs.num.into_modulo(r)).residue()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ibig::{ubig, UBig};

    #[test]
    fn create_valid() -> Result<()> {
        let e = Element::new("1", 10);

        assert!(e.is_ok());
        Ok(())
    }

    #[test]
    fn create_invalid() -> Result<()> {
        let e = Element::new(&P.with(|p| -> UBig { p + 1 }).to_string(), 10);

        assert!(e.is_err());
        Ok(())
    }

    #[test]
    fn equal() -> Result<()> {
        let e1 = Element::new("2", 10)?;
        let e2 = Element::new("2", 10)?;

        assert!(e1 == e2);
        Ok(())
    }

    #[test]
    fn unequal() -> Result<()> {
        let e1 = Element::new("2", 10)?;
        let e2 = Element::new("2", 10)?;

        assert!(e1 != e2);
        Ok(())
    }

    #[test]
    fn neg_exp() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new(
            "91823464351457740977292442922341431300552291046805345244117967493152195424759",
            10,
        )?;

        assert_eq!(e1.pow("-3", 10)?, e2);
        Ok(())
    }

    #[test]
    fn pos_exp() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new("343", 10)?;

        assert_eq!(e1.pow("3", 10)?, e2);
        Ok(())
    }

    #[test]
    fn sqrt() -> Result<()> {
        let e1 = Element::new("452345243", 10)?;
        let e2 = Element::new(
            "60918528521079593676672830288299404598099605221950081497966121269789262591401",
            10,
        )?;

        assert_eq!(e1.sqrt(), e2);
        Ok(())
    }

    #[test]
    fn is_zero() -> Result<()> {
        let e = Element::new("0", 10)?;

        assert!(e.is_zero());
        Ok(())
    }

    #[test]
    fn is_not_zero() -> Result<()> {
        let e = Element::new("1", 10)?;

        assert!(!e.is_zero());
        Ok(())
    }

    #[test]
    fn get_num() -> Result<()> {
        let e = Element::new("3", 10)?;

        assert_eq!(e.num(), ubig!(3));
        Ok(())
    }

    #[test]
    fn add_elems() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new("12", 10)?;
        let e3 = Element::new("19", 10)?;

        assert_eq!(e1 + e2, e3);
        Ok(())
    }

    #[test]
    fn add_ref_elems() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new("15", 10)?;
        let e3 = Element::new("22", 10)?;

        assert_eq!(&e1 + &e2, e3);
        Ok(())
    }

    #[test]
    fn add_ref_reg_elems() -> Result<()> {
        let e1 = Element::new("8", 10)?;
        let e2 = Element::new("12", 10)?;
        let e3 = Element::new("20", 10)?;

        assert_eq!(e1 + &e2, e3);
        Ok(())
    }

    #[test]
    fn sub_elems() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new("12", 10)?;
        let e3 = Element::new(
            "115792089237316195423570985008687907853269984665640564039457584007908834671658",
            10,
        )?;

        assert_eq!(e1 - e2, e3);
        Ok(())
    }

    #[test]
    fn sub_ref_elems() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new("12", 10)?;
        let e3 = Element::new(
            "115792089237316195423570985008687907853269984665640564039457584007908834671658",
            10,
        )?;

        assert_eq!(&e1 - &e2, e3);
        Ok(())
    }

    #[test]
    fn sub_ref_reg_elems() -> Result<()> {
        let e1 = Element::new("7", 10)?;
        let e2 = Element::new("12", 10)?;
        let e3 = Element::new(
            "115792089237316195423570985008687907853269984665640564039457584007908834671658",
            10,
        )?;

        assert_eq!(e1 - &e2, e3);
        Ok(())
    }

    #[test]
    fn mul_elems() -> Result<()> {
        let e1 = Element::new("3", 10)?;
        let e2 = Element::new("12", 10)?;
        let e3 = Element::new("36", 10)?;

        assert_eq!(e1 * e2, e3);
        Ok(())
    }

    #[test]
    fn mul_ref_elems() -> Result<()> {
        let e1 = Element::new("3", 10)?;
        let e2 = Element::new("13", 10)?;
        let e3 = Element::new("39", 10)?;

        assert_eq!(&e1 * &e2, e3);
        Ok(())
    }

    #[test]
    fn mul_ref_reg_elems() -> Result<()> {
        let e1 = Element::new("4", 10)?;
        let e2 = Element::new("13", 10)?;
        let e3 = Element::new("52", 10)?;

        assert_eq!(&e1 * e2, e3);
        Ok(())
    }

    #[test]
    fn div_elems() -> Result<()> {
        let e1 = Element::new("2", 10)?;
        let e2 = Element::new("7", 10)?;
        let e3 = Element::new(
            "82708635169511568159693560720491362752335703332600402885326845719934881908331",
            10,
        )?;

        assert_eq!(e1 / e2, e3);
        Ok(())
    }

    #[test]
    fn div_ref_elems() -> Result<()> {
        let e1 = Element::new("2", 10)?;
        let e2 = Element::new("7", 10)?;
        let e3 = Element::new(
            "82708635169511568159693560720491362752335703332600402885326845719934881908331",
            10,
        )?;

        assert_eq!(&e1 / &e2, e3);
        Ok(())
    }

    #[test]
    fn div_ref_reg_elems() -> Result<()> {
        let e1 = Element::new("2", 10)?;
        let e2 = Element::new("7", 10)?;
        let e3 = Element::new(
            "82708635169511568159693560720491362752335703332600402885326845719934881908331",
            10,
        )?;

        assert_eq!(&e1 / e2, e3);
        Ok(())
    }
}
