use std::marker::PhantomData;

use super::checker::PrimeChecker;

pub struct Combine<N, F, S> {
    first: F,
    second: S,
    _phantom: PhantomData<N>
}

impl<N, F ,S> Combine<N, F, S> {
    pub fn new(first: F, second: S) -> Self {
        Combine {
            first,
            second,
            _phantom: Default::default()
        }
    }
}

impl<N, F, S> PrimeChecker for Combine<N, F, S>
    where F: PrimeChecker<Value=N>,
          S: PrimeChecker<Value=N>
{
    type Value = N;

    fn check(&self, value: &Self::Value) -> bool {
        self.first.check(value) && self.second.check(value)
    }
}