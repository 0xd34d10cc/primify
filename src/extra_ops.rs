pub trait ModPow {
    fn mod_pow(&self, exponent: &Self, modulus: &Self) -> Self;
}

pub trait RandInt {
    fn get_rand_below(max: &Self) -> Self;
}

pub trait ToStrRadix {
    fn to_str(&self, radix: u8) -> String;
}