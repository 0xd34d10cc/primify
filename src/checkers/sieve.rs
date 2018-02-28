use std::marker::PhantomData;
use std::ops::Rem;

use primal;
use num::traits::Zero;

use super::checker::PrimeChecker;

pub struct SieveChecker<N> {
   sieve:  primal::Sieve,
   _phantom: PhantomData<N>
}

impl<N> SieveChecker<N> {
    pub fn new(size: usize) -> Self {
        SieveChecker {
            sieve: primal::Sieve::new(size),
            _phantom: Default::default()
        }
    }
}

impl<N> PrimeChecker for SieveChecker<N>
    where N: Zero + Eq,
         for <'a> &'a N: Rem<usize, Output=N>
{
    type Value = N;

    fn check(&self, value: &Self::Value) -> bool {
        let zero = Self::Value::zero();

        self.sieve
            .primes_from(2)
            .all(|prime| value % prime != zero)
    } 
}