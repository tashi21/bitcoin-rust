use {
    super::{
        constants::{SECP256K1_GENERATOR_POINT, SECP256K1_ORDER, SECP256K1_ORDER_RING},
        point::Point,
        signature::Signature,
    },
    anyhow::Result,
    hmac::{Hmac, Mac},
    ibig::{modular::IntoModulo, UBig},
    sha2::Sha256,
    std::fmt::{self, Display, Formatter},
};

pub struct PrivateKey {
    point: Point,
    e: UBig,
}

impl PrivateKey {
    /// Create a new Private Key with the given secret
    pub fn new(e: UBig) -> Self {
        Self {
            point: SECP256K1_GENERATOR_POINT.with(|g| e.clone() * g),
            e,
        }
    }

    /// Generate a Signature from a message hash using the Private Key
    pub fn sign(&self, z: UBig) -> Result<Signature> {
        let k = self.deterministic_k(z.clone())?;
        let r = SECP256K1_GENERATOR_POINT.with(|g| (k.clone() * g).x()); // x co coridinate of R point
        let s = SECP256K1_ORDER_RING.with(|ring| {
            ((z + r.clone() * self.e.clone()).into_modulo(ring) / k.into_modulo(ring)).residue()
        }); // s = (z + r * secret) / k

        let s = SECP256K1_ORDER.with(|o| if s > o / 2 { o - s } else { s }); // correct s if greater than half the order

        Ok(Signature::new(r, s))
    }

    /// Generate a unique, deterministic k for a given message hash (`z`) and Private Key (`self`).
    fn deterministic_k(&self, z: UBig) -> Result<UBig> {
        let mut k = b"\x00".repeat(32); // initial k value
        let mut v = b"\x01".repeat(32); // initial v value
        let z_bytes = SECP256K1_ORDER.with(|o| {
            if z > *o {
                (z - o).to_be_bytes()
            } else {
                z.to_be_bytes()
            }
        });
        let secret_bytes = self.e.to_be_bytes();

        type Sha256Hmac = Hmac<Sha256>;

        let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
        mac.update(
            [
                v.as_slice(),
                b"\x00".as_slice(),
                secret_bytes.as_slice(),
                z_bytes.as_slice(),
            ]
            .concat()
            .as_slice(),
        );
        k = mac.finalize().into_bytes().to_vec(); // update k

        let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
        mac.update(v.as_slice());
        v = mac.finalize().into_bytes().to_vec(); // update v

        let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
        mac.update(
            [
                v.as_slice(),
                b"\x01".as_slice(),
                secret_bytes.as_slice(),
                z_bytes.as_slice(),
            ]
            .concat()
            .as_slice(),
        );
        k = mac.finalize().into_bytes().to_vec(); // update k

        let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
        mac.update(v.as_slice());
        v = mac.finalize().into_bytes().to_vec(); // update v

        loop {
            let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
            mac.update(v.as_slice());
            v = mac.finalize().into_bytes().to_vec(); // update v

            let candidate = UBig::from_be_bytes(v.as_slice());
            if candidate >= UBig::from(1_u8) && SECP256K1_ORDER.with(|o| candidate < *o) {
                return Ok(candidate);
            }

            let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
            mac.update([v.as_slice(), b"\x00".as_slice()].concat().as_slice());
            k = mac.finalize().into_bytes().to_vec(); // update k

            let mut mac = Sha256Hmac::new_from_slice(k.as_slice())?;
            mac.update(v.as_slice());
            v = mac.finalize().into_bytes().to_vec(); // update v
        }
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.point)
    }
}

#[cfg(test)]
mod test {}
