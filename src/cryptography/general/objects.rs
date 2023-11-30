
// symmetric ciphers interfaces
pub trait AEADinterface {
    fn encrypt(plaintext: Vec<u8>, associated_data: Vec<u8>);
    fn decrypt(ciphertext: Vec<u8>, associated_data: Vec<u8>);
}


// asymmetric ciphers interfaces
pub trait RSAinterface {
    fn encrypt(plaintext: Vec<u8>);
    fn decrypt(ciphertext: Vec<u8>);
}

pub trait DHinterface {
    fn 
}