extern crate ramp;

use rand;
pub use self::ramp::Int;

use extra_ops::{ModPow, RandInt, ToStrRadix};

impl ModPow for Int {
    fn mod_pow(&self, exponent: &Self, modulus: &Self) -> Self {
        self.pow_mod(exponent, modulus)
    }
}

impl RandInt for Int {
    fn get_rand_below(max: &Self) -> Self {
        use self::ramp::RandomInt;

        rand::thread_rng().gen_uint_below(max)
    }
}

impl ToStrRadix for Int {
    fn to_str(&self, radix: u8) -> String {
        self.to_str_radix(radix, false)
    }
}