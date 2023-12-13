use bigdecimal::BigDecimalRef;
use num_bigint::BigInt;
use num_bigint::Sign;

use crate::{cryptography::asymmetric::interfaces::interfaces::{RSAinterface}, algebras::FiniteField::{classes::Zmod::Zmod, instances::Zmod_instance::{self, ZmodInstance}}, numbers::numbers::Class, arith::primes::get_strong_prime};

pub struct RSA {
    public_keys: Vec<(BigInt, BigInt)>, // n, e
    private_keys: Vec<(BigInt, BigInt, BigInt)>,// p, q, d
    primary_key: usize
}

impl RSAinterface for RSA {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        // convert bytes to BigInt as big endian
        let value: BigInt = BigInt::from_bytes_be(Sign::Plus, &plaintext);
        let (n,e) = self.public_keys[self.primary_key].clone();
        let ciphertext = value.modpow(&e, &n);
        let (_s, c) = ciphertext.to_bytes_be();
        c
    }

    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        let value: BigInt = BigInt::from_bytes_be(Sign::Plus, &ciphertext);
        let (p,q, d) = self.private_keys[self.primary_key].clone();
        let plaintext = value.modpow(&d, &(p*q));
        let (_s, p) = plaintext.to_bytes_be();
        p
    }
}

impl RSA {
    pub fn init(n_bits: u32) -> Self {
        let p: BigInt = get_strong_prime(n_bits, true);
        let q: BigInt = get_strong_prime(n_bits, true);
        let n: BigInt = p.clone()*q.clone();
        let phin: BigInt = (p.clone()-1)*(q.clone()-1);
        let phin_field: Zmod = Zmod::new(Some(phin));
        let e: ZmodInstance = phin_field.apply(BigInt::from(65537));
        let d: ZmodInstance = e.clone().inverse();

        let mut public_keys: Vec<(BigInt, BigInt)> = Vec::new();        
        public_keys.push( (n, e.get_bigint_value()) );
        let mut private_keys: Vec<(BigInt, BigInt, BigInt)> = Vec::new();        
        private_keys.push( (p, q, d.get_bigint_value()) );

        RSA { public_keys: public_keys, private_keys: private_keys, primary_key: 0}
    }

    pub fn refresh_keys(&mut self, n_bits: u32) {
        let p: BigInt = get_strong_prime(n_bits, true);
        let q: BigInt = get_strong_prime(n_bits, true);
        let n: BigInt = p.clone()*q.clone();
        let phin: BigInt = (p.clone()-1)*(q.clone()-1);
        let phin_field: Zmod = Zmod::new(Some(phin));
        let e: ZmodInstance = phin_field.apply(BigInt::from(65537));
        let d: ZmodInstance = e.clone().inverse();

               
        self.public_keys.push( (n, e.get_bigint_value()) );
        self.private_keys.push( (p, q, d.get_bigint_value()) );
        self.primary_key = self.primary_key + 1
    } 

    pub fn get_current_public_key(&self) -> (BigInt, BigInt) {
        self.public_keys[self.primary_key].clone()
    }

    pub fn get_current_private_key(&self) -> (BigInt, BigInt, BigInt) {
        self.private_keys[self.primary_key].clone()
    }

    // if the supplied index is outside the vector size it raises Exception
    // TRUE: set new primary key
    // FALSE: operation not permitted
    pub fn set_primary_key(&mut self, new_primary_key: usize) -> bool {
        if new_primary_key > self.private_keys.len() {
            return false;
        }

        self.primary_key = new_primary_key;
        return true;
    }

}