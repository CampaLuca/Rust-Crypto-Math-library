// asymmetric ciphers interfaces
pub trait RSAinterface {
    fn encrypt(&self, plaintext: Vec<u8>) -> Vec<u8>;
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8>;
}

pub trait DHinterface {
    fn get_session_key();
}

pub trait SignatureInterface {
    fn sign(&self, message: Vec<u8>) -> Vec<u8>;
    fn verify(&self, message: Vec<u8>, signature: Vec<u8>) -> bool;
}