use rand::Rng;
use num_bigint::{BigInt, BigUint, RandomBits};

pub fn get_random_bigint(length: u64) -> BigInt {
    let mut rng = rand::thread_rng();
    let signed: BigInt = rng.sample(RandomBits::new(length));
    signed
}

pub fn get_random_biguint(length: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let unsigned: BigUint = rng.sample(RandomBits::new(length));
    unsigned
}


