use super::combine::Combine;

pub trait PrimeChecker {
    type Value;

    // returns 
    //  false if |value| is not a prime
    //  true otherwise
    fn check(&self, value: &Self::Value) -> bool;

    fn combine<C>(self, checker: C) -> Combine<Self::Value, Self, C> 
        where Self: Sized,
              C: PrimeChecker<Value=Self::Value>
    {
        Combine::new(self, checker)
    }
}
