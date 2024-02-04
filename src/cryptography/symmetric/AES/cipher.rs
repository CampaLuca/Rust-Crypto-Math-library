use crate::cryptography::symmetric::{primitives::aes::AES_KEY_SIZE, AES::aes_functions::*};

pub fn encrypt(message: &mut Vec<u8>, key: Vec<u8>, rounds: usize, key_size: AES_KEY_SIZE) {

    let keys: Vec<Vec<u8>> = match key_size {
        AES_KEY_SIZE::AES_128 => AES_128_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
        AES_KEY_SIZE::AES_192 => AES_192_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
        AES_KEY_SIZE::AES_256 => AES_256_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
        _ => AES_128_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
    };
    // add Key_0 to the message
    add(message, &keys[0]);
    
    for i in 0..rounds {
        forward_round(message);
        add(message, &keys[i+1] );
    }
}


pub fn decrypt(message: &mut Vec<u8>, key: Vec<u8>, rounds: usize, key_size: AES_KEY_SIZE) {
    let keys: Vec<Vec<u8>> = match key_size {
        AES_KEY_SIZE::AES_128 => AES_128_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
        AES_KEY_SIZE::AES_192 => AES_192_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
        AES_KEY_SIZE::AES_256 => AES_256_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
        _ => AES_128_KEY_EXPANDER::get_n_rounds_keys(rounds, key),
    };
  
    add(message, &keys[rounds]);
    

    for i in (0..rounds).rev() {
        backward_round(message);
        add(message, &keys[i] );
    }
}