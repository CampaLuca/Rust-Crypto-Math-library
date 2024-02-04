use bigdecimal::BigDecimalRef;
use num_bigint::BigInt;
use num_bigint::Sign;

use crate::numbers::classes::ZZ::ZZ;
use crate::numbers::instances::ZZ_instance::ZZinstance;
use crate::{cryptography::asymmetric::interfaces::interfaces::{PKIinterface}, algebras::FiniteField::{classes::Zmod::Zmod, instances::Zmod_instance::{self, ZmodInstance}}, numbers::numbers::Class, arith::primes::get_strong_prime};

pub struct RSA {
    pub public_keys: Vec<(ZZinstance, ZZinstance)>, // n, e
    pub private_keys: Vec<(ZZinstance, ZZinstance, ZZinstance)>,// p, q, d
    pub primary_key: usize
}

impl PKIinterface for RSA {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        // convert bytes to BigInt as big endian
        let value: BigInt = BigInt::from_bytes_be(Sign::Plus, &plaintext);
        let (n,e) = self.public_keys[self.primary_key].clone();
        let ciphertext = value.modpow(&e.value, &n.value);
        let (_s, c) = ciphertext.to_bytes_be();
        c
    }

    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        let value: BigInt = BigInt::from_bytes_be(Sign::Plus, &ciphertext);
        let (p,q, d) = self.private_keys[self.primary_key].clone();
        let (n,e) = self.public_keys[self.primary_key].clone();
        let plaintext = value.modpow(&d.value, &n.value);
        let (_s, p) = plaintext.to_bytes_be();
        p
    }
}

impl RSA {
    pub fn init(n_bits: u32) -> Self {
        let zz: ZZ = ZZ::new();
        let p: ZZinstance = zz.new_instance(get_strong_prime(n_bits, false));
        let q: ZZinstance = p.next_prime();//zz.new_instance(get_strong_prime(n_bits, false));
        let n: ZZinstance = p.clone()*q.clone();
        let phin: ZZinstance = (p.clone()-1)*(q.clone()-1);

        let phin_field: Zmod = Zmod::new(Some(phin));
        let e: ZmodInstance = phin_field.apply(BigInt::from(65537));
        let d: ZmodInstance = e.clone().inverse();


        let mut public_keys: Vec<(ZZinstance, ZZinstance)> = Vec::new();        
        public_keys.push( (n, e.get_bigint_value()) );
        let mut private_keys: Vec<(ZZinstance, ZZinstance, ZZinstance)> = Vec::new();        
        private_keys.push( (p, q, d.get_bigint_value()) );

        RSA { public_keys: public_keys, private_keys: private_keys, primary_key: 0}
    }

    pub fn refresh_keys(&mut self, n_bits: u32) {
        let zz: ZZ = ZZ::new();
        let p: ZZinstance = zz.new_instance(get_strong_prime(n_bits, false));
        
        let q: ZZinstance = p.next_prime();//zz.new_instance(get_strong_prime(n_bits, false));
        let n: ZZinstance = p.clone()*q.clone();
        let phin: ZZinstance = (p.clone()-1)*(q.clone()-1);
        let phin_field: Zmod = Zmod::new(Some(phin));
        let e: ZmodInstance = phin_field.apply(BigInt::from(65537));
        let d: ZmodInstance = e.clone().inverse();

               
        self.public_keys.push( (n, e.get_bigint_value()) );
        self.private_keys.push( (p, q, d.get_bigint_value()) );
        self.primary_key = self.primary_key + 1
    } 

    pub fn get_current_public_key(&self) -> (ZZinstance, ZZinstance) {
        self.public_keys[self.primary_key].clone()
    }

    pub fn get_current_private_key(&self) -> (ZZinstance, ZZinstance, ZZinstance) {
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