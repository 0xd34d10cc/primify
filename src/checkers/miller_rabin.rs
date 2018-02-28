use std::marker::PhantomData;

use num::traits::NumAssignRef;
use num::integer::Integer;
use super::checker::PrimeChecker;
use extra_ops::{ModPow, RandInt};


pub struct MillerRabinChecker<N> {
    witnessess: usize,
    _phantom: PhantomData<N>

}

impl<N> MillerRabinChecker<N> {
    pub fn new(witnessess: usize) -> Self {
        MillerRabinChecker {
            witnessess, 
            _phantom: Default::default()
        }
    }
}

impl<N> PrimeChecker for MillerRabinChecker<N> 
    where N: NumAssignRef + ModPow + RandInt + Integer + Clone
{
    type Value = N;

    fn check(&self, value: &Self::Value) -> bool {
        miller_rabin(value, self.witnessess)
    }
}

fn extract_pow2<N>(n: N) -> (N, N) 
    where N: NumAssignRef + Integer 
{
    let one = N::one();
    let two = N::one() + N::one();

    let mut d = n;
    let mut r = N::zero();
    
    while d.is_even() {
        d /= &two;
        r += &one;
    }

    (d, r)
}

fn miller_rabin<N>(n: &N, k: usize) -> bool 
    where N: NumAssignRef + ModPow + RandInt + Integer + Clone 
{
    let zero = N::zero();
    let one = N::one();
    let two = N::one() + N::one();
    
    // corner case that leads to infinite loop in generate_candidate()
    {
        let three = two.clone() + N::one();
        if n == &three {
            return true;
        }
    }


    let n_1 = {
        let mut n = n.clone();
        n -= &one;
        n
    };

    let (d, r) = extract_pow2(n_1.clone());

    let generate_candidate = |n| {
        loop {
            let num = N::get_rand_below(n);
            if num != one && num != zero {
                break num;
            }
        }
    };

    (0..k).into_iter()
        .map(|_| generate_candidate(&n_1))
        .all(|a| {
            let mut x = a.mod_pow(&d, &n);
            let is_witness = {
                if x == one || x == n_1 {
                    true
                } else {
                    let mut r = r.clone(); 
                    loop {
                        x = x.mod_pow(&two, &n);
                        if x == one {
                            break false;
                        }

                        if x == n_1 {
                            break true;
                        }

                        if r == zero {
                            break false;
                        }
                        r -= &one;
                    }
                }
            };

            is_witness
        })
}