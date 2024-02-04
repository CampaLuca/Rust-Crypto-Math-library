// file for prime generators

// get_prime

// get_strong_prime

// get_mersenne_prime

// get_4p_1_prime

use num_bigint::{BigInt, BigUint, RandBigInt};
use num_integer::Integer;
use crate::numbers::{numbers::{Instance, Class}, classes::ZZ::ZZ, instances::ZZ_instance::ZZinstance};
use rand;

use super::random::get_random_bigint;

pub fn is_prime(n: BigUint) -> bool {
    // Translated from
    // https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test#Perl

    if n < BigUint::from(2u32) {
        return false;
    }

    if n == BigUint::from(2u32) || n == BigUint::from(3u32) || n == BigUint::from(5u32) {
        return true;
    }

    if (&n % BigUint::from(2u32)) == BigUint::from(0u32) {
        return false;
    }

    let n_sub = n.clone() - BigUint::from(1u32);
    let mut exponent = n_sub.clone();
    let mut trials = 0;

    while (&exponent % BigUint::from(2u32)) == BigUint::from(0u32) {
        exponent /= 2u32;
        trials += 1;
    }

    'LOOP: for i in 1..((n.to_string().len()) + 2) {
        let mut result = bmodpow(&(BigUint::from(2u32) + i), &exponent, &n);

        if result == BigUint::from(1u32) || result == n_sub {
            continue;
        }

        for _ in 1..trials {
            result = result.pow(2) % &n;

            if result == BigUint::from(1u32) {
                return false;
            }

            if result == n_sub {
                continue 'LOOP;
            }
        }

        return false;
    }

    true
}

fn bmodpow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    // Translated from
    // http://search.cpan.org/~pjacklam/Math-BigInt-1.999810/lib/Math/BigInt.pm#Arithmetic_methods

    if *base == BigUint::from(0u32) {
        return match *exponent == BigUint::from(0u32) {
            true => BigUint::from(1u32),
            false => BigUint::from(0u32),
        };
    }

    if *modulus == BigUint::from(1u32) {
        return BigUint::from(0u32);
    }

    let exponent_in_binary = exponent.to_radix_le(2);
    let mut my_base = base.clone();
    let mut result = BigUint::from(1u32);

    for next_bit in exponent_in_binary {
        if next_bit == 1 {
            result = (result * my_base.clone()) % modulus;
        }

        my_base = my_base.pow(2) % modulus;
    }

    result
}


fn decompose(n: &BigUint) -> (BigUint, BigUint) {
    let one = BigUint::from(1u32);
    let ref two = BigUint::from(2u32);
    let mut d: BigUint = (n - 1u8).clone();
    let mut s: BigUint = BigUint::from(0u32);

    while &d % two == one {
        d /= two;
        s += 1u8;
    }

    (d, s)
}

fn miller_rabin_prime(n: &BigUint, k: u32) -> bool {
    let mut rng = rand::thread_rng();
    let aux_two: BigUint = BigUint::from(2u32);
    let aux_one: BigUint = BigUint::from(1u32);
    let aux_zero: BigUint = BigUint::from(0u32);

    // to be updated with a table of values that reduce the complexity of the miller rabin test

    for _i in 0..k {
        let (ref d, ref s) = decompose(n);
        let a: BigUint = rng.gen_biguint_range(&BigUint::from(2u32), &(n-2u8));
        let mut x = a.modpow(d, n);
        let mut count: BigUint = BigUint::from(0u32);
        while count < *s {
            let mut y = x.modpow(&aux_two, n);
            if y == aux_one && x != aux_one && x != (n-1u8) {
                return false;
            }
            x = y;
            count += 1u8;
        }
        if x != aux_one {
            return false;
        }
    }

    return true;

}



fn baillie_psw_probabilistic_prime(n: &BigInt) -> bool{
    let aux_two: BigInt = BigInt::from(2);
    let aux_one: BigInt = BigInt::from(1);
    let aux_zero: BigInt = BigInt::from(0);


    if n.clone() == aux_two {
        return true;
    }
    if (n.clone() % aux_two) == aux_zero {
        return false;
    } else {
        /* 16294579238595022365 = 3*5*7*11*13*17*19*23*29*31*37*41*43*47*53
        *  7145393598349078859 = 59*61*67*71*73*79*83*89*97*101 */
        if !is_coprime(n, &BigInt::from(16294579238595022365u64)) && !is_coprime(n, &BigInt::from(7145393598349078859u64)) {
            return false;
        } 

        /* 4127218095 = 3*5*7*11*13*17*19*23*37
            * 3948078067 = 29*31*41*43*47*53
            * 4269855901 = 59*83*89*97*101
            * 1673450759 = 61*67*71*73*79 */
        if !is_coprime(n, &BigInt::from(4127218095i64)) && !is_coprime(n, &BigInt::from(3948078067i64)) && !is_coprime(n, &BigInt::from(4269855901i64)) && !is_coprime(n, &BigInt::from(1673450759i64)) {
            return false;
        } 

        return true; //return is2psp(n) && is_lucas_psp(n)

    }

}


// fn is2psp(n: &BigInt) {

// }


pub fn is_pseudoprime(a: BigInt, algorithm: bool) -> bool {
    if algorithm {
        miller_rabin_prime(&(a.to_biguint().unwrap()), 100u32) // set k = 100 by default
    } else {
        baillie_psw_probabilistic_prime(&a)
    }
}


fn max<T>(a: T, b: T) -> T  where T: Instance + Clone + PartialEq + PartialOrd {
    if a > b {
        return a;
    } else {
        return b;
    }
}

pub fn get_strong_prime_in_range(ubound: BigInt, proof: bool, lower_bound: BigInt) -> BigInt {
    let zz_class = ZZ::new();
    let n: ZZinstance  = zz_class.apply(ubound);
    let lbound: ZZinstance = zz_class.apply(lower_bound);
    let mut result = BigInt::from(2);

    if n < zz_class.apply(BigInt::from(2)) {
        panic!("n must be greater than or equal to 2");
    }

    if n < lbound {
        panic!("n must be at least lbound");
    } else if n == zz_class.apply(BigInt::from(2)) {
        return n.value;
    }

    let lbound: ZZinstance = max::<ZZinstance>(lbound, zz_class.apply(BigInt::from(2)));

    if lbound > zz_class.apply(BigInt::from(2)) {
        if lbound == zz_class.apply(BigInt::from(3)) || n <= zz_class.apply(BigInt::from(2))*lbound.clone()-zz_class.apply(BigInt::from(2)) {
            // check for Betrand's postulate (proved by Chebyshev)
            if lbound < zz_class.apply(BigInt::from(25)) || n <= zz_class.apply(BigInt::from(6))*lbound.clone()/zz_class.apply(BigInt::from(5)) {
                // see J.nagura
                if lbound < zz_class.apply(BigInt::from(2070760)) || n <= zz_class.apply(BigInt::from(16598))*lbound.clone()/zz_class.apply(BigInt::from(16597)) {
                    // see L. Schoenfeld, Math. Comp 1976
                    let mut smallest_prime: ZZinstance = n.clone();
                    if proof {
                        smallest_prime = (lbound.clone()-zz_class.apply(BigInt::from(1))).next_prime();
                    } else {
                        smallest_prime = (lbound.clone()-zz_class.apply(BigInt::from(1))).next_probable_prime();
                    }
                    if smallest_prime > n {
                        panic!("There are no primes between those limits")
                    }

                    result = smallest_prime.value.clone();
                }
            }
        }
    }

    result

    

}



pub fn get_strong_prime(nbits: u32, proof: bool) -> BigInt {
    let one: BigInt = BigInt::from(1);
    let upper_bound: BigInt = one.clone()<<nbits;

    let lower_bound: BigInt = (one.clone()<<(nbits-1)) - get_random_bigint((nbits-1) as u64);

    let mut result: BigInt = BigInt::from(2);
    let zz_class = ZZ::new();
    let n: ZZinstance  = zz_class.apply(upper_bound);
    let lbound: ZZinstance = zz_class.apply(lower_bound);

    if n < zz_class.apply(BigInt::from(2)) {
        panic!("n must be greater than or equal to 2");
    }

    if n < lbound {
        panic!("n must be at least lbound");
    } else if n == zz_class.apply(BigInt::from(2)) {
        return n.value;
    }


    let mut lbound: ZZinstance = max::<ZZinstance>(lbound, zz_class.apply(BigInt::from(2)));


    if lbound > zz_class.apply(BigInt::from(2)) {
        if lbound == zz_class.apply(BigInt::from(3)) || n <= zz_class.apply(BigInt::from(2))*lbound.clone()-zz_class.apply(BigInt::from(2)) {
            // check for Betrand's postulate (proved by Chebyshev)
            if lbound < zz_class.apply(BigInt::from(25)) || n <= zz_class.apply(BigInt::from(6))*lbound.clone()/zz_class.apply(BigInt::from(5)) {
                // see J.nagura
                if lbound < zz_class.apply(BigInt::from(2070760)) || n <= zz_class.apply(BigInt::from(16598))*lbound.clone()/zz_class.apply(BigInt::from(16597)) {
                    // see L. Schoenfeld, Math. Comp 1976
                    let mut smallest_prime: ZZinstance = n.clone();
                    if proof {
                        smallest_prime = (lbound.clone()-zz_class.apply(BigInt::from(1))).next_prime();
                    } else {
                        smallest_prime = (lbound.clone()-zz_class.apply(BigInt::from(1))).next_probable_prime();
                    }


                    if smallest_prime > n {
                        panic!("There are no primes between those limits")
                    }

                    lbound = lbound.class.into_inner().apply(smallest_prime.value.clone());
                }
            }
        }
    }

    result = lbound.next_prime().value;

    result
    

}

fn is_coprime(n: &BigInt, m: &BigInt) -> bool {
    let one: BigInt = BigInt::from(1);
    if n.gcd(m) != one {
        return false;
    }

    true
}