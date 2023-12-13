// symmetric ciphers interfaces
pub trait AEADinterface {
    fn encrypt(plaintext: Vec<u8>, associated_data: Vec<u8>);
    fn decrypt(ciphertext: Vec<u8>, associated_data: Vec<u8>);
}

pub trait MACinterface {
    fn get_authentication_tag(plaintext: Vec<u8>);
    fn verify(plaintext: Vec<u8>, authentication_tag: Vec<u8>);
}
