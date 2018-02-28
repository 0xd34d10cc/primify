extern crate gmp;

use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};
use std::ops::{Neg, ShlAssign, ShrAssign, BitAndAssign};
use std::fmt::{self, Display};

use self::gmp::mpz::Mpz as Inner;
use num::traits::{Num, One, Zero};
use num::integer::Integer;

use extra_ops::{ModPow, RandInt, ToStrRadix};

// All this boilerplate exist because gmp::Mpz don't have `impl Num`.
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub struct Mpz(Inner);

impl Mpz {
    pub fn mod_pow(&self, exp: &Self, modulus: &Self) -> Self {
        Mpz(self.0.powm(&exp.0, &modulus.0))
    }

    pub fn to_str_radix(&self, radix: u8) -> String {
        self.as_inner().to_str_radix(radix)
    }

    pub fn as_inner(&self) -> &Inner {
        &self.0
    }
}

impl Display for Mpz {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<Inner> for Mpz {
    fn from(inner: Inner) -> Self {
        Mpz(inner)
    }
}

impl Into<Inner> for Mpz {
    fn into(self) -> Inner {
        self.0
    }
}

impl Add for Mpz {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self
    }
}

impl AddAssign for Mpz {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<'a> AddAssign<&'a Mpz> for Mpz {
    fn add_assign(&mut self, rhs: &Mpz) {
        self.0 += &rhs.0;
    }
}

impl Sub for Mpz {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0 -= rhs.0;
        self
    }
}

impl SubAssign for Mpz {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<'a> SubAssign<&'a Mpz> for Mpz {
    fn sub_assign(&mut self, rhs: &Mpz) {
        self.0 -= &rhs.0;
    }
}

impl Mul for Mpz {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 *= rhs.0;
        self
    }
}

impl MulAssign for Mpz {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<'a> MulAssign<&'a Mpz> for Mpz {
    fn mul_assign(&mut self, rhs: &Mpz) {
        self.0 *= &rhs.0;
    }
}

impl Div for Mpz {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.0 /= rhs.0;
        self
    }
}

impl DivAssign for Mpz {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<'a> DivAssign<&'a Mpz> for Mpz {
    fn div_assign(&mut self, rhs: &Mpz) {
        self.0 /= &rhs.0;
    }
}

impl Rem for Mpz {
    type Output = Self;

    fn rem(mut self, rhs: Self) -> Self::Output {
        self.0 %= rhs.0;
        self
    }
}

impl<'a> Rem<usize> for &'a Mpz {
    type Output = Mpz;

    fn rem(self, rhs: usize) -> Self::Output {
        Mpz(&self.0 % (rhs as u64))
    }
}

impl RemAssign for Mpz {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl<'a> RemAssign<&'a Mpz> for Mpz {
    fn rem_assign(&mut self, rhs: &Mpz) {
        self.0 %= &rhs.0;
    }
}

impl Zero for Mpz {
    fn zero() -> Self {
        Mpz(Inner::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Mpz {
    fn one() -> Self {
        Mpz(Inner::one())
    }
}

impl Integer for Mpz {
    fn is_even(&self) -> bool {
        !self.is_odd()
    }

    fn is_odd(&self) -> bool {
        self.0.tstbit(0)        
    }

    fn div_floor(&self, rhs: &Self) -> Self {
        self.0.div_floor(rhs.as_inner()).into()
    } 

    fn mod_floor(&self, rhs: &Self) -> Self {
        self.0.mod_floor(rhs.as_inner()).into()
    }

    fn gcd(&self, rhs: &Self) -> Self {
        self.0.gcd(rhs.as_inner()).into()
    }

    fn lcm(&self, rhs: &Self) -> Self {
        self.0.lcm(rhs.as_inner()).into()
    }

    fn divides(&self, rhs: &Self) -> bool {
        self.0.divides(rhs.as_inner()).into()
    }

    fn is_multiple_of(&self, rhs: &Self) -> bool {
        self.0.is_multiple_of(rhs.as_inner()).into()
    }

    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        let div = self.0.clone().div(rhs.as_inner()).into();
        let rem = self.0.clone().rem(rhs.as_inner()).into();
        (div, rem)
    }
}

impl Num for Mpz {
    type FromStrRadixErr = self::gmp::mpz::ParseMpzError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Mpz(Inner::from_str_radix(s, radix as u8)?))
    }
}

impl Neg for Mpz {
    type Output = Mpz;

    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}

impl ShlAssign<usize> for Mpz {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl ShrAssign<usize> for Mpz {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl<'a> BitAndAssign<&'a Mpz> for Mpz {
    fn bitand_assign(&mut self, rhs: &Mpz) {
        self.0 &= &rhs.0;
    }
}

impl ModPow for Mpz {
    fn mod_pow(&self, exponent: &Self, modulus: &Self) -> Self {
        self.mod_pow(exponent, modulus)
    }
}

impl RandInt for Mpz {
    fn get_rand_below(max: &Self) -> Self {
        use self::gmp::rand::RandState;
        use std::cell::RefCell;

        thread_local!(static GEN: RefCell<RandState> = RefCell::new(RandState::new()));

        let randinner = GEN.with(|gen| {
            let inner = max.as_inner();
            let rand = gen.borrow_mut().urandom(inner);
            rand
        });

        Mpz::from(randinner)
    }
}

impl ToStrRadix for Mpz {
    fn to_str(&self, radix: u8) -> String {
        self.as_inner().to_str_radix(radix)
    }
}

impl From<usize> for Mpz {
    fn from(val: usize) -> Self {
        Mpz(Inner::from(val as  u64))
    }
}