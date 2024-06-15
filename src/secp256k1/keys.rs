use {
    super::{
        constants::{G, N, N_RING},
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
    pub fn new(e: &str, radix: u32) -> Result<Self> {
        let e = UBig::from_str_radix(e, radix)?;
        Ok(Self {
            point: G.with(|g| &e * g),
            e,
        })
    }

    /// Generate a Signature from a message hash (in hexadecimal) using the Private Key
    pub fn sign(&self, z: &str) -> Result<Signature> {
        let z = UBig::from_str_radix(z, 16)?;
        let k = self.deterministic_k(&z)?; // generate deterministic k for given z
        let r = G.with(|g| (&k * g).x()); // x co coordinate of R point
        let s = N_RING.with(|n_ring| {
            ((z + &r * &self.e).into_modulo(n_ring) / k.into_modulo(n_ring)).residue()
        }); // s = (z + r * secret) / k

        let s = N.with(|n| if s > n / 2 { n - s } else { s }); // correct s if greater than half the order

        Ok(Signature::new(r, s))
    }

    /// Generate a unique, deterministic k for a given message hash (`z`) and Private Key (`self`).
    fn deterministic_k(&self, z: &UBig) -> Result<UBig> {
        let mut k = b"\x00".repeat(32); // initial k value
        let mut v = b"\x01".repeat(32); // initial v value
        let z_bytes = N.with(|o| {
            if z > o {
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
            if candidate >= UBig::from(1_u8) && N.with(|o| candidate < *o) {
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
