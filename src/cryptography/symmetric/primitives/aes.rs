use rand::Rng;
use rayon::prelude::*;

use crate::cryptography::{symmetric::{modes::modes::Modes, interfaces::interfaces::{AESinterface, AESfactory}, AES::{aes_functions, cipher::{encrypt, decrypt, self}}}, padding::padding::Paddings};




fn random_byte_array(length: usize) -> Vec<u8> {
    let mut byte_array: Vec<u8> = Vec::new();
    for _i in 0..length {
        byte_array.push(rand::thread_rng().gen::<u8>())
    }

    byte_array
}

fn byte_array_xor(m1: &mut Vec<u8>, m2: &Vec<u8>) {
    
    for i in 0..m1.len() {
        m1[i] ^= m2[i];
    }
    
}

pub struct aes_factory {

}

impl AESfactory for aes_factory {
    fn init(mode: Modes, padding: Paddings) -> Box<dyn AESinterface> {
        let result: Box<dyn AESinterface> = match mode { 
            Modes::CBC=>AES_CBC::init(),
            Modes::CTR=>AES_CTR_with_preprocessing::init(),
            // Modes::OFB=>AES_OFB::init(),
            // Modes::GCM=>AES_GCM::init(),
            Modes::ECB=>AES_ECB::init(),
            _=>AES::init()           
          };

          result
    }
}


// simple AES
pub struct AES {
    keys: Vec<Vec<u8>>, 
    primary_key: usize
}

impl AESinterface for AES {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        if plaintext.len() > 16 {
            panic!("Simple AES performs one block encryption at a time");
        }

        let mut ciphertext = plaintext.clone();
        
        encrypt( &mut ciphertext, self.keys[self.primary_key].clone(), Some(16));
        ciphertext
    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        if ciphertext.len() > 16 {
            panic!("Simple AES performs one block encryption at a time");
        }

        let mut padded_plaintext = ciphertext.clone();
        decrypt(&mut padded_plaintext, self.keys[self.primary_key].clone(), Some(16));
        padded_plaintext
    }
}

impl AES {
    pub fn init() -> Box<dyn AESinterface> {
        let key: Vec<u8> = random_byte_array(16);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key);
        Box::new(AES { keys: keys, primary_key: 0})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(16);
        self.keys.push(key);
        self.primary_key = self.primary_key + 1;
    }
}
//end simple AES

// AES_CTR with pre-processing: Byte Stream is generated alongside the initialization.
// In this way IV is of 15 bytes and counter of 8 bits. Then, at maximum 256 = 2^8 blocks can be encrypted. 
pub struct AES_CTR_with_preprocessing {
    keys: Vec<Vec<u8>>, 
    primary_key: usize,
    current_byte_stream: Vec<Vec<u8>>
}

impl AESinterface for AES_CTR_with_preprocessing {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        // divide plaintext into blocks
        let blocks;
        if plaintext.len() % 16 == 0 {
            blocks = plaintext.len() / 16;
        } else {
            blocks = (plaintext.len() / 16)+1;
        }

        let mut blocks_plaintext: Vec<Vec<u8>> = Vec::new();
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                if i*16+j >= plaintext.len() {
                    break;
                }
                temp_vector.push(plaintext[i*16+j]);
            }
            blocks_plaintext.push(temp_vector);
        }

        blocks_plaintext.par_iter_mut().enumerate().for_each(|(i, x)| byte_array_xor(x, &self.current_byte_stream[i]));
        self.refresh_byte_stream();
        blocks_plaintext.concat()

    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        let mut blocks = 0;
        if ciphertext.len() % 16 == 0 {
            blocks = ciphertext.len() / 16;
        } else {
            blocks = (ciphertext.len() / 16)+1;
        }

        let mut blocks_ciphertext: Vec<Vec<u8>> = Vec::new();
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                if i*16+j >= ciphertext.len() {
                    break;
                }
                temp_vector.push(ciphertext[i*16+j]);
            }
            blocks_ciphertext.push(temp_vector);
        }

        blocks_ciphertext.par_iter_mut().enumerate().for_each(|(i, x)| byte_array_xor(x, &self.current_byte_stream[i]));
        blocks_ciphertext.concat()
    }
}

impl AES_CTR_with_preprocessing {
    pub fn init() -> Box<dyn AESinterface> {
        let key: Vec<u8> = random_byte_array(16);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key.clone());

        let iv: Vec<u8> = random_byte_array(15);
        let byte_stream: Vec<Vec<u8>> = AES_CTR_with_preprocessing::get_byte_stream(key, iv);
        Box::new(AES_CTR_with_preprocessing { keys: keys, primary_key: 0, current_byte_stream: byte_stream})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(16);
        self.keys.push(key);
        self.primary_key = self.primary_key + 1;
    }

    fn refresh_byte_stream(&mut self) {
        let iv: Vec<u8> = random_byte_array(15);
        let byte_stream: Vec<Vec<u8>> = AES_CTR_with_preprocessing::get_byte_stream(self.keys[self.primary_key].clone(), iv);
        self.current_byte_stream = byte_stream;
    }

    // the computation of the byte_stream is parallelized
    fn get_byte_stream(key: Vec<u8>, nonce_iv: Vec<u8>) -> Vec<Vec<u8>> {
        let mut byte_stream_container: Vec<Vec<u8>> = Vec::new();
        let counter: u8 = rand::thread_rng().gen::<u8>();
        for _i in 0..256 {
            let mut temp_vector: Vec<u8> = nonce_iv.clone();
            temp_vector.push(counter);
            byte_stream_container.push(temp_vector);
        }

        byte_stream_container.par_iter_mut().for_each(|x| encrypt(x, key.clone(), Some(16)));
        byte_stream_container
    }
}
// end AES_CTR


pub struct AES_CBC {
    keys: Vec<Vec<u8>>, 
    primary_key: usize
}

impl AESinterface for AES_CBC {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        // random iv
        let iv: Vec<u8> = random_byte_array(16);
        
        // pad plaintext 
        let padded_plaintext: Vec<u8> = Vec::new(); //pad(plaintext);

        // dividing plaintext into blocks
        let mut blocks_padded_plaintext: Vec<Vec<u8>> = Vec::new();
        let blocks = padded_plaintext.len() / 16;
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                temp_vector.push(padded_plaintext[i*16+j]);
            }
            blocks_padded_plaintext.push(temp_vector);
        }

        let mut current_iv = iv.clone();
        for r in 0..blocks {
            byte_array_xor(&mut blocks_padded_plaintext[r], &current_iv);
            encrypt(&mut blocks_padded_plaintext[r], self.keys[self.primary_key].clone(), Some(16));
            current_iv = blocks_padded_plaintext[r].clone();
        }


        // put encrypted blocks into one vector
        let mut ciphertext_container: Vec<u8> = Vec::new();

        // first of all put the iv at the beginning of the ciphertext
        for i in 0..16 {
            ciphertext_container.push(iv[i]);
        }

        for i in 0..blocks {
            for j in 0..16 {
                ciphertext_container.push(blocks_padded_plaintext[i][j]);
            }
        }

        ciphertext_container


        
    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        let iv: Vec<u8> = ciphertext[0..16].to_vec();
        let ciphertext_container: Vec<u8> = ciphertext[16..].to_vec();
        
        // dividing ciphertext into blocks
        let mut blocks_ciphertext: Vec<Vec<u8>> = Vec::new();
        let blocks = ciphertext_container.len() / 16;
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                temp_vector.push(ciphertext_container[i*16+j]);
            }
            blocks_ciphertext.push(temp_vector);
        }

        let mut current_iv = iv.clone();
        for r in 0..blocks {
            let temp_iv: Vec<u8> = blocks_ciphertext[r].clone();
            decrypt(&mut blocks_ciphertext[r], self.keys[self.primary_key].clone(), Some(16));
            byte_array_xor(&mut blocks_ciphertext[r], &current_iv);
            current_iv = temp_iv;
        }


        // put encrypted blocks into one vector
        
        blocks_ciphertext.concat()



    }
}

impl AES_CBC {
    pub fn init() -> Box<dyn AESinterface> {
        let key: Vec<u8> = random_byte_array(16);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key);
        Box::new(AES_CBC { keys: keys, primary_key: 0})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(16);
        self.keys.push(key);
        self.primary_key = self.primary_key + 1;
    }
}
//end AES_CBC

pub struct AES_ECB {
    keys: Vec<Vec<u8>>, 
    primary_key: usize
}

impl AESinterface for AES_ECB {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        // compute for how many block we should encrypt
        let mut padded_plaintext_blocks: Vec<Vec<u8>> = Vec::new();
        let padded_plaintext: Vec<u8> = Vec::new(); //pad(plaintext);

        let blocks = padded_plaintext.len() / 16;
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                temp_vector.push(padded_plaintext[i*16+j]);
            }
            padded_plaintext_blocks.push(temp_vector);
        }

        padded_plaintext_blocks.par_iter_mut().for_each(|x| encrypt(x, self.keys[self.primary_key].clone(), Some(16)));

        padded_plaintext_blocks.concat()
    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        //dividing ciphertexts in blocks
        let mut ciphertext_container: Vec<Vec<u8>> = Vec::new();
        let blocks = ciphertext.len() / 16;
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                temp_vector.push(ciphertext[i*16+j]);
            }
            ciphertext_container.push(temp_vector);
        }

        ciphertext_container.par_iter_mut().for_each(|x| decrypt(x, self.keys[self.primary_key].clone(), Some(16)));
        ciphertext_container.concat()// unpad plaintext
    }
}


// that implementation is parallelized
impl AES_ECB {
    pub fn init() -> Box<dyn AESinterface> {
        let key: Vec<u8> = random_byte_array(16);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key);
        Box::new(AES_ECB { keys: keys, primary_key: 0})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(16);
        self.keys.push(key);
        self.primary_key = self.primary_key + 1;
    }
}
// end AES ECB

// pub struct AES_GCM {
//     keys: Vec<Vec<u8>>, 
//     primary_key: usize
// }

// impl AESinterface for AES_GCM {
//     fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        
//     }

//     fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        
//     }
// }

// impl AES_GCM {
//     fn init() {}
// }
// // end AES_GCM

// pub struct AES_OFB {
//     keys: Vec<Vec<u8>>, 
//     primary_key: usize
// }

// impl AESinterface for AES_OFB {
//     fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        
//     }

//     fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        
//     }
// }

// impl AES_OFB {
//     fn init() {}
// }
// // end AES OFB




