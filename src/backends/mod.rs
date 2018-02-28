#[cfg(feature = "num_backend")]
mod bigint;
#[cfg(feature = "num_backend")]
pub type BigInt = bigint::BigUint;

#[cfg(feature = "ramp_backend")]
mod ramp;
#[cfg(feature = "ramp_backend")]
pub type BigInt = ramp::Int;

#[cfg(feature = "gmp_backend")]
mod mpz;
#[cfg(feature = "gmp_backend")]
pub type BigInt = mpz::Mpz;