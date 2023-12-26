use crate::{algebras::{Rings::instances::PolynomialRing_instance::PolynomialRingInstance, FiniteField::instances::Zmod_instance::ZmodInstance}, matrices::vector::Vector};

// asymmetric ciphers interfaces
pub trait PKIinterface {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8>;
}

pub trait LatticeBased_PKIinterface {
    fn encrypt(&self, plaintext: Vec<u8>) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>);
    fn decrypt(&self, u: Vector<PolynomialRingInstance<ZmodInstance>>, v: PolynomialRingInstance<ZmodInstance>) -> Vec<u8>;
}

pub trait KEMinterface {
    fn encapsulate(&self, bytes_length: usize) -> (Vector<PolynomialRingInstance<ZmodInstance>>, PolynomialRingInstance<ZmodInstance>);
}

pub trait KeyExchange {
    fn get_session_key();
}

pub trait SignatureInterface {
    fn sign(&self, message: Vec<u8>) -> Vec<u8>;
    fn verify(&self, message: Vec<u8>, signature: Vec<u8>) -> bool;
}