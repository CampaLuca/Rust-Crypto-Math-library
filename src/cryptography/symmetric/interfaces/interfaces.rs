// symmetric ciphers interfaces

use crate::cryptography::{symmetric::modes::modes::Modes, padding::padding::Paddings};

// refactoring with ! 
pub trait AESfactory {
    fn init(mode: Modes, padding: Paddings) -> Box<dyn AESinterface>;
}




pub trait AESinterface {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8>;
    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8>;
}
pub trait AEADinterface {
    fn encrypt(&self, plaintext: Vec<u8>, associated_data: Vec<u8>);
    fn decrypt(&self, ciphertext: Vec<u8>, associated_data: Vec<u8>);
}

pub trait MACinterface {
    fn get_authentication_tag(&self, plaintext: Vec<u8>);
    fn verify(&self, plaintext: Vec<u8>, authentication_tag: Vec<u8>);
}
