use crate::cryptography::symmetric::AES::aes_functions::*;

pub fn encrypt(message: &mut Vec<u8>, key: Vec<u8>, rounds: Option<usize>) {
    let mut k = key.clone();
    // add Key_0 to the message
    add(message, &k);
    
    for i in 0..rounds.unwrap_or(16) {
        next_key_from_expansion(&mut k, i);
        forward_round(message);
        add(message, &k );
    }
}


pub fn decrypt(message: &mut Vec<u8>, key: Vec<u8>, rounds: Option<usize>) {
    let mut k = key.clone();
    // add Key_0 to the message
    get_ith_key_from_expansion(&mut k, 0, 16);
   
    add(message, &k);
    

    for i in (0..rounds.unwrap_or(16)).rev() {
        prec_key_from_expansion(&mut k, i);
        backward_round(message);
        add(message, &k );
    }
}