use rand;
pub use num::bigint::BigUint;

use extra_ops::{ModPow, RandInt, ToStrRadix};

impl ModPow for BigUint {
    fn mod_pow(&self, exponent: &Self, modulus: &Self) -> Self {
        self.modpow(exponent, modulus)
    }
}

impl RandInt for BigUint {
    fn get_rand_below(max: &Self) -> Self {
        use num::bigint::RandBigInt;

        rand::thread_rng().gen_biguint_below(max)
    }
}

impl ToStrRadix for BigUint {
    fn to_str(&self, radix: u8) -> String {
        self.to_str_radix(radix as u32)
    }
}