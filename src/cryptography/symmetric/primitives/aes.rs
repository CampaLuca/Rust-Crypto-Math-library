use core::num;

use num_traits::ToBytes;
use rand::Rng;
use rayon::prelude::*;

use crate::cryptography::{symmetric::{modes::modes::Modes, interfaces::interfaces::{AESinterface, AESfactory}, AES::{aes_functions, cipher::{encrypt, decrypt, self}}}, padding::padding::{GenericPadding, Pad, Paddings}};

#[derive(Clone)]
pub enum AES_KEY_SIZE {
    AES_128,
    AES_192,
    AES_256
}

fn random_byte_array(length: usize) -> Vec<u8> {
    let mut byte_array: Vec<u8> = Vec::new();
    for _i in 0..length {
        byte_array.push(rand::thread_rng().gen::<u8>())
    }

    byte_array
}

fn inplace_byte_array_xor(m1: &mut Vec<u8>, m2: &Vec<u8>) {
    
    for i in 0..m1.len() {
        m1[i] ^= m2[i];
    }
    
}




pub struct aes_factory {

}

impl AESfactory for aes_factory {
    fn init(mode: Modes, padding: Paddings, key_size: AES_KEY_SIZE) -> Box<dyn AESinterface> {
        let result: Box<dyn AESinterface> = match mode { 
            Modes::CBC=>AES_CBC::init(padding, key_size),
            Modes::CTR=>AES_CTR_with_preprocessing::init(key_size),
            // Modes::OFB=>AES_OFB::init(),
            // Modes::GCM=>AES_GCM::init(),
            Modes::ECB=>AES_ECB::init(padding, key_size),
            _=>AES::init(key_size)           
          };

          result
    }
}


// simple AES
pub struct AES {
    keys: Vec<Vec<u8>>, 
    primary_key: usize,
    key_size: AES_KEY_SIZE,
    n_rounds: usize,
    number_of_key_bytes: usize

}

impl AESinterface for AES {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        if plaintext.len() > 16 {
            panic!("Simple AES performs one block encryption at a time");
        }

        let mut ciphertext = plaintext.clone();
        
        encrypt( &mut ciphertext, self.keys[self.primary_key].clone(), self.n_rounds, self.key_size.clone());
        ciphertext
    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        if ciphertext.len() > 16 {
            panic!("Simple AES performs one block encryption at a time");
        }

        let mut padded_plaintext = ciphertext.clone();
        decrypt(&mut padded_plaintext, self.keys[self.primary_key].clone(), self.n_rounds, self.key_size.clone());
        padded_plaintext
    }
}

impl AES {
    pub fn init(key_size: AES_KEY_SIZE) -> Box<dyn AESinterface> {
        

        let n_rounds: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 10,
            AES_KEY_SIZE::AES_192 => 12,
            AES_KEY_SIZE::AES_256 => 14,
            _ => 10
        };

        let number_of_key_bytes: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 16,
            AES_KEY_SIZE::AES_192 => 24,
            AES_KEY_SIZE::AES_256 => 32,
            _ => 16
        };

        let key: Vec<u8> = random_byte_array(number_of_key_bytes);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key);

        Box::new(AES { keys: keys, primary_key: 0, key_size: key_size, n_rounds: n_rounds, number_of_key_bytes: number_of_key_bytes})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(self.number_of_key_bytes);
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
    current_byte_stream: Vec<Vec<u8>>, 
    current_iv: Vec<u8>, // unique for each encryption
    nonce: Vec<u8>, // unique for each security association (chosen at init at random)
    key_size: AES_KEY_SIZE,
    n_rounds: usize,
    number_of_key_bytes: usize

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

        
        blocks_plaintext.par_iter_mut().enumerate().for_each(|(i, x)| inplace_byte_array_xor(x, &self.current_byte_stream[i]));
        blocks_plaintext.insert(0, self.current_iv.clone());
        self.refresh_byte_stream();
        blocks_plaintext.concat()

    }

    fn decrypt(&mut self, ciphertext: Vec<u8>) -> Vec<u8> {
        let iv: Vec<u8> = ciphertext[0..8].to_vec();
        let ciphertext_container: Vec<u8> = ciphertext[8..].to_vec();
        let mut blocks = 0;
        if ciphertext_container.len() % 16 == 0 {
            blocks = ciphertext_container.len() / 16;
        } else {
            blocks = (ciphertext_container.len() / 16)+1;
        }

        let mut blocks_ciphertext: Vec<Vec<u8>> = Vec::new();

        

        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                if i*16+j >= ciphertext_container.len() {
                    break;
                }
                temp_vector.push(ciphertext_container[i*16+j]);
            }

            
            blocks_ciphertext.push(temp_vector);
            
        }

        
        let byte_stream = AES_CTR_with_preprocessing::get_byte_stream(self.keys[self.primary_key].clone(), iv.clone(), self.nonce.clone(), self.n_rounds, self.key_size.clone());

        blocks_ciphertext.par_iter_mut().enumerate().for_each(|(i, x)| inplace_byte_array_xor(x, &byte_stream[i]));
        blocks_ciphertext.concat()
    }
}

impl AES_CTR_with_preprocessing {
    pub fn init(key_size: AES_KEY_SIZE) -> Box<dyn AESinterface> {
       

        let n_rounds: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 10,
            AES_KEY_SIZE::AES_192 => 12,
            AES_KEY_SIZE::AES_256 => 14,
            _ => 10
        };

        let number_of_key_bytes: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 16,
            AES_KEY_SIZE::AES_192 => 24,
            AES_KEY_SIZE::AES_256 => 32,
            _ => 16
        };

        let key: Vec<u8> = random_byte_array(number_of_key_bytes);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key.clone());

        let iv: Vec<u8> = random_byte_array(8);
        let nonce: Vec<u8> = random_byte_array(4);
        let byte_stream: Vec<Vec<u8>> = AES_CTR_with_preprocessing::get_byte_stream(key, iv.clone(), nonce.clone(), n_rounds, key_size.clone());
        
        
        Box::new(AES_CTR_with_preprocessing { keys: keys, primary_key: 0, current_byte_stream: byte_stream, current_iv: iv, nonce: nonce, key_size: key_size.clone(), n_rounds: n_rounds, number_of_key_bytes: number_of_key_bytes})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(self.number_of_key_bytes);
        self.keys.push(key);
        self.primary_key = self.primary_key + 1;
    }

    fn refresh_byte_stream(&mut self) {
        let iv: Vec<u8> = random_byte_array(8);
        let byte_stream: Vec<Vec<u8>> = AES_CTR_with_preprocessing::get_byte_stream(self.keys[self.primary_key].clone(), iv.clone(), self.nonce.clone(), self.n_rounds, self.key_size.clone());
        self.current_byte_stream = byte_stream;
        self.current_iv = iv.clone();
    }

    // the computation of the byte_stream is parallelized
    pub fn get_byte_stream(key: Vec<u8>, iv: Vec<u8>, nonce: Vec<u8>, n_rounds: usize, key_size: AES_KEY_SIZE) -> Vec<Vec<u8>> {
        let mut byte_stream_container: Vec<Vec<u8>> = Vec::new();
        let mut counter: u32 = 0;
        for _i in 0..256 {
            let mut temp_vector: Vec<u8> = nonce.clone();
            temp_vector.append(&mut iv.clone());
            temp_vector.append(&mut counter.to_be_bytes().to_vec());
            byte_stream_container.push(temp_vector);
            counter += 1;
        }

        byte_stream_container.par_iter_mut().for_each(|x| encrypt(x, key.clone(), n_rounds, key_size.clone()));
        byte_stream_container
    }
}
// end AES_CTR


pub struct AES_CBC {
    keys: Vec<Vec<u8>>, 
    primary_key: usize,
    padding: GenericPadding,
    key_size: AES_KEY_SIZE,
    n_rounds: usize,
    number_of_key_bytes: usize


}

impl AESinterface for AES_CBC {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        // random iv
        let iv: Vec<u8> = random_byte_array(16);
        
        // pad plaintext 
        
        let padded_plaintext: Vec<u8> = self.padding.pad(plaintext, 16);

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
            inplace_byte_array_xor(&mut blocks_padded_plaintext[r], &current_iv);
            encrypt(&mut blocks_padded_plaintext[r], self.keys[self.primary_key].clone(), self.n_rounds, self.key_size.clone());
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
            decrypt(&mut blocks_ciphertext[r], self.keys[self.primary_key].clone(), self.n_rounds, self.key_size.clone());
            inplace_byte_array_xor(&mut blocks_ciphertext[r], &current_iv);
            current_iv = temp_iv;
        }


        // put encrypted blocks into one vector
        
        self.padding.unpad(blocks_ciphertext.concat(), true)



    }
}

impl AES_CBC {
    pub fn init(padding: Paddings, key_size: AES_KEY_SIZE) -> Box<dyn AESinterface> {
        

        let n_rounds: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 10,
            AES_KEY_SIZE::AES_192 => 12,
            AES_KEY_SIZE::AES_256 => 14,
            _ => 10
        };

        let number_of_key_bytes: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 16,
            AES_KEY_SIZE::AES_192 => 24,
            AES_KEY_SIZE::AES_256 => 32,
            _ => 16
        };

        let key: Vec<u8> = random_byte_array(number_of_key_bytes);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key);

        Box::new(AES_CBC { keys: keys, primary_key: 0, padding: GenericPadding::init(padding), key_size: key_size, n_rounds: n_rounds, number_of_key_bytes: number_of_key_bytes})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(self.number_of_key_bytes);
        self.keys.push(key);
        self.primary_key = self.primary_key + 1;
    }
}
//end AES_CBC

pub struct AES_ECB {
    keys: Vec<Vec<u8>>, 
    primary_key: usize,
    padding: GenericPadding,
    key_size: AES_KEY_SIZE,
    n_rounds: usize,
    number_of_key_bytes: usize


}

impl AESinterface for AES_ECB {
    fn encrypt(&mut self, plaintext: Vec<u8>) -> Vec<u8> {
        // compute for how many block we should encrypt
        let mut padded_plaintext_blocks: Vec<Vec<u8>> = Vec::new();
        let padded_plaintext: Vec<u8> = self.padding.pad(plaintext, 16);

        let blocks = padded_plaintext.len() / 16;
        for i in 0..blocks {
            let mut temp_vector: Vec<u8> = Vec::new();
            for j in 0..16 {
                temp_vector.push(padded_plaintext[i*16+j]);
            }
            padded_plaintext_blocks.push(temp_vector);
        }

        padded_plaintext_blocks.par_iter_mut().for_each(|x| encrypt(x, self.keys[self.primary_key].clone(), self.n_rounds, self.key_size.clone()));

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

        ciphertext_container.par_iter_mut().for_each(|x| decrypt(x, self.keys[self.primary_key].clone(), self.n_rounds, self.key_size.clone()));
        self.padding.unpad(ciphertext_container.concat(), true)
    }
}


// that implementation is parallelized
impl AES_ECB {
    pub fn init(padding: Paddings, key_size: AES_KEY_SIZE) -> Box<dyn AESinterface> {
        

        let n_rounds: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 10,
            AES_KEY_SIZE::AES_192 => 12,
            AES_KEY_SIZE::AES_256 => 14,
            _ => 10
        };

        let number_of_key_bytes: usize = match key_size {
            AES_KEY_SIZE::AES_128 => 16,
            AES_KEY_SIZE::AES_192 => 24,
            AES_KEY_SIZE::AES_256 => 32,
            _ => 16
        };

        let key: Vec<u8> = random_byte_array(number_of_key_bytes);
        let mut keys: Vec<Vec<u8>> = Vec::new();
        keys.push(key);
        
        Box::new(AES_ECB { keys: keys, primary_key: 0, padding: GenericPadding::init(padding), key_size: key_size, n_rounds: n_rounds, number_of_key_bytes: number_of_key_bytes})
    }

    pub fn refresh_key(&mut self) {
        let key: Vec<u8> = random_byte_array(self.number_of_key_bytes);
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




