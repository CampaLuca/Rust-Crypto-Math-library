use num_bigint::BigInt;

use crate::{algebras::{FiniteField::instances::Zmod_instance::ZmodInstance, Rings::instances::PolynomialRing_instance::PolynomialRingInstance}, matrices::vector::Vector, numbers::instances::ZZ_instance::ZZinstance};

pub trait FHE<T> {
    fn homomorphic_addition(&self, C1: T, C2: T) -> T;
    fn homomorphic_multiplication(&self, C1: T, C2: T) -> T;
}

pub trait BFV_PKI {
    fn key_gen(&mut self);
    fn encrypt(&self, plaintext: BigInt) -> (PolynomialRingInstance<ZmodInstance>, PolynomialRingInstance<ZmodInstance>);
    fn decrypt(&self, ciphertext: Vec<PolynomialRingInstance<ZmodInstance>>) -> BigInt;
}
