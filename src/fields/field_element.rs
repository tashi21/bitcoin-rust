use {
    super::errors::FieldErrors,
    anyhow::{bail, Result},
    ibig::{
        modular::{IntoModulo, ModuloRing},
        IBig, UBig,
    },
    std::{
        fmt::{self, Display, Formatter},
        ops::{Add, Div, Mul, Sub},
    },
};

#[derive(Debug, Eq)]
pub struct FieldElement {
    /// Number in the field
    num: UBig,
    /// Prime of the field
    prime: UBig,
    /// Ring of the Field
    ring: ModuloRing,
}

impl FieldElement {
    /// Create a new Field Element with the given prime as the field size
    pub fn new(num: UBig, prime: UBig) -> Result<Self> {
        if num >= prime {
            bail!(FieldErrors::NotInRange(num, prime));
        }

        Ok(Self {
            num,
            ring: ModuloRing::new(&prime),
            prime,
        })
    }

    /// Check if two Field Elements are from the same field
    pub fn same_field(&self, other: &Self) -> bool {
        self.prime == other.prime
    }

    /// Raises self to the power of `exp` and returns the new computed value
    pub fn pow(&self, exp: IBig) -> Self {
        Self {
            num: (&self.num)
                .into_modulo(&self.ring)
                .pow_signed(&exp)
                .residue(),
            ring: ModuloRing::new(&self.prime),
            prime: self.prime.clone(),
        }
    }

    /// Return the zero of the field of this field element
    pub fn zero(&self) -> Self {
        Self {
            num: UBig::from(0_u8),
            ring: ModuloRing::new(&self.prime),
            prime: self.prime.clone(),
        }
    }

    /// Return the prime of the Field Element
    pub fn prime(&self) -> UBig {
        self.prime.clone()
    }

    /// Return the Field Element number
    pub fn num(&self) -> UBig {
        self.num.clone()
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}{}", self.num, self.prime)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.num == other.num
    }
}

impl Clone for FieldElement {
    fn clone(&self) -> Self {
        Self {
            num: self.num.clone(),
            ring: ModuloRing::new(&self.prime),
            prime: self.prime.clone(),
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    /// Add two fields only if they have the same prime
    fn add(self, rhs: Self) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs));
        }
        Self::Output {
            num: (self.num.into_modulo(&self.ring) + rhs.num.into_modulo(&self.ring)).residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Add for &FieldElement {
    type Output = FieldElement;

    /// Add two fields only if they have the same prime
    fn add(self, rhs: Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs.clone()));
        }
        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring)
                + rhs.num.clone().into_modulo(&self.ring))
            .residue(),
            ring: ModuloRing::new(&self.prime),
            prime: self.prime.clone(),
        }
    }
}

impl Add<&FieldElement> for FieldElement {
    type Output = Self;

    /// Add two fields only if they have the same prime
    fn add(self, rhs: &Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs.clone()));
        }
        Self::Output {
            num: (self.num.into_modulo(&self.ring) + rhs.num.clone().into_modulo(&self.ring))
                .residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Add<FieldElement> for &FieldElement {
    type Output = FieldElement;

    /// Add two fields only if they have the same prime
    fn add(self, rhs: Self::Output) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs));
        }
        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring) + rhs.num.into_modulo(&self.ring))
                .residue(),
            prime: rhs.prime,
            ring: rhs.ring,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    /// Subtract two fields only if they have the same prime
    fn sub(self, rhs: Self) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs));
        }
        Self::Output {
            num: (self.num.into_modulo(&self.ring) - rhs.num.into_modulo(&self.ring)).residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Sub for &FieldElement {
    type Output = FieldElement;

    /// Subtract two fields only if they have the same prime
    fn sub(self, rhs: Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs.clone()));
        }
        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring)
                - rhs.num.clone().into_modulo(&self.ring))
            .residue(),
            ring: ModuloRing::new(&self.prime()),
            prime: self.prime.clone(),
        }
    }
}

impl Sub<&FieldElement> for FieldElement {
    type Output = Self;

    /// Subtract two fields only if they have the same prime
    fn sub(self, rhs: &Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs.clone()));
        }
        Self::Output {
            num: (self.num.into_modulo(&self.ring) - rhs.num.clone().into_modulo(&self.ring))
                .residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Sub<FieldElement> for &FieldElement {
    type Output = FieldElement;

    /// Subtract two fields only if they have the same prime
    fn sub(self, rhs: Self::Output) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs));
        }
        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring) - rhs.num.into_modulo(&self.ring))
                .residue(),
            prime: rhs.prime,
            ring: rhs.ring,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    /// Multiply two fields only if they have the same prime
    fn mul(self, rhs: Self) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs));
        }
        Self::Output {
            num: (self.num.into_modulo(&self.ring) * rhs.num.into_modulo(&self.ring)).residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Mul for &FieldElement {
    type Output = FieldElement;

    /// Multiply two fields only if they have the same prime
    fn mul(self, rhs: Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs.clone()));
        }
        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring)
                * rhs.num.clone().into_modulo(&self.ring))
            .residue(),
            ring: ModuloRing::new(&self.prime()),
            prime: self.prime.clone(),
        }
    }
}

impl Mul<&FieldElement> for FieldElement {
    type Output = Self;

    /// Multiply two fields only if they have the same prime
    fn mul(self, rhs: &Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs.clone()));
        }
        Self::Output {
            num: (self.num.into_modulo(&self.ring) * rhs.num.clone().into_modulo(&self.ring))
                .residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Mul<FieldElement> for &FieldElement {
    type Output = FieldElement;

    /// Multiply two fields only if they have the same prime
    fn mul(self, rhs: Self::Output) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs));
        }
        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring) * rhs.num.into_modulo(&self.ring))
                .residue(),
            prime: rhs.prime,
            ring: rhs.ring,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    /// Divide two fields only if they have the same prime
    fn div(self, rhs: Self) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs));
        }

        Self::Output {
            num: (self.num.into_modulo(&self.ring) / rhs.num.into_modulo(&self.ring)).residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Div for &FieldElement {
    type Output = FieldElement;

    /// Divide two fields only if they have the same prime
    fn div(self, rhs: Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs.clone()));
        }

        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring)
                / rhs.num.clone().into_modulo(&self.ring))
            .residue(),
            ring: ModuloRing::new(&self.prime()),
            prime: self.prime.clone(),
        }
    }
}

impl Div<&FieldElement> for FieldElement {
    type Output = Self;

    /// Divide two fields only if they have the same prime
    fn div(self, rhs: &Self) -> Self::Output {
        if !self.same_field(rhs) {
            panic!("{}", FieldErrors::NotSameField(self, rhs.clone()));
        }

        Self::Output {
            num: (self.num.into_modulo(&self.ring) / rhs.num.clone().into_modulo(&self.ring))
                .residue(),
            prime: self.prime,
            ring: self.ring,
        }
    }
}

impl Div<FieldElement> for &FieldElement {
    type Output = FieldElement;

    /// Divide two fields only if they have the same prime
    fn div(self, rhs: Self::Output) -> Self::Output {
        if !self.same_field(&rhs) {
            panic!("{}", FieldErrors::NotSameField(self.clone(), rhs));
        }

        Self::Output {
            num: (self.num.clone().into_modulo(&self.ring) / rhs.num.into_modulo(&self.ring))
                .residue(),
            prime: rhs.prime,
            ring: rhs.ring,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ibig::{ibig, ubig};

    #[test]
    fn create_valid_field_elem() -> Result<()> {
        let elem = FieldElement::new(ubig!(1), ubig!(5));

        assert!(elem.is_ok());
        Ok(())
    }

    #[test]
    fn create_invalid_field_elem() -> Result<()> {
        let elem = FieldElement::new(ubig!(6), ubig!(5));

        assert!(elem.is_err());
        Ok(())
    }

    #[test]
    fn equal_fields() -> Result<()> {
        let f1 = FieldElement::new(ubig!(2), ubig!(5))?;
        let f2 = FieldElement::new(ubig!(2), ubig!(5))?;

        assert_eq!(f1, f2);
        Ok(())
    }

    #[test]
    fn unequal_fields_num() -> Result<()> {
        let f1 = FieldElement::new(ubig!(2), ubig!(5))?;
        let f2 = FieldElement::new(ubig!(3), ubig!(5))?;

        assert_ne!(f1, f2);
        Ok(())
    }

    #[test]
    fn unequal_fields_prime() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(5))?;
        let f2 = FieldElement::new(ubig!(3), ubig!(7))?;

        assert_ne!(f1, f2);
        Ok(())
    }

    #[test]
    fn get_zero() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(5))?;
        let f2 = f1.zero();

        assert_eq!(
            f2,
            FieldElement {
                num: ubig!(0),
                prime: ubig!(5),
                ring: ModuloRing::new(&ubig!(5)),
            }
        );
        assert!(f1.same_field(&f2));
        assert!(f2.same_field(&f1));
        Ok(())
    }

    #[test]
    fn get_prime() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(5))?;

        assert_eq!(f1.prime(), ubig!(5));
        Ok(())
    }

    #[test]
    fn get_num() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(5))?;

        assert_eq!(f1.num(), ubig!(3));
        Ok(())
    }

    #[test]
    fn check_same_field_true() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(5))?;
        let f2 = FieldElement::new(ubig!(4), ubig!(5))?;

        assert!(f1.same_field(&f2));
        assert!(f2.same_field(&f1));
        Ok(())
    }

    #[test]
    fn check_same_field_false() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(7))?;
        let f2 = FieldElement::new(ubig!(4), ubig!(5))?;

        assert!(!f1.same_field(&f2));
        assert!(!f2.same_field(&f1));
        Ok(())
    }

    #[test]
    fn add_fields() -> Result<()> {
        let f1 = FieldElement::new(ubig!(7), ubig!(13))?;
        let f2 = FieldElement::new(ubig!(12), ubig!(13))?;
        let f3 = FieldElement::new(ubig!(6), ubig!(13))?;

        assert_eq!(f1 + f2, f3);
        Ok(())
    }

    #[test]
    fn sub_fields() -> Result<()> {
        let f1 = FieldElement::new(ubig!(7), ubig!(13))?;
        let f2 = FieldElement::new(ubig!(12), ubig!(13))?;
        let f3 = FieldElement::new(ubig!(8), ubig!(13))?;

        assert_eq!(f1 - f2, f3);
        Ok(())
    }

    #[test]
    fn mul_fields() -> Result<()> {
        let f1 = FieldElement::new(ubig!(3), ubig!(13))?;
        let f2 = FieldElement::new(ubig!(12), ubig!(13))?;
        let f3 = FieldElement::new(ubig!(10), ubig!(13))?;

        assert_eq!(f1 * f2, f3);
        Ok(())
    }

    #[test]
    fn div_fields() -> Result<()> {
        let f1 = FieldElement::new(ubig!(2), ubig!(19))?;
        let f2 = FieldElement::new(ubig!(7), ubig!(19))?;
        let f3 = FieldElement::new(ubig!(3), ubig!(19))?;

        assert_eq!(f1 / f2, f3);
        Ok(())
    }

    #[test]
    fn exp_fields() -> Result<()> {
        let f1 = FieldElement::new(ubig!(7), ubig!(13))?;
        let f2 = FieldElement::new(ubig!(8), ubig!(13))?;

        assert_eq!(f1.pow(ibig!(-3)), f2);
        Ok(())
    }
}
