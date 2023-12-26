use rand::Rng;

pub enum Paddings {
    ANSI_X9_23,
    ISO_10126,
    PKCS7,
    ISO_IEC_7816_4
}





pub trait Pad {
    fn pad(message: Vec<u8>, block_size: usize) -> Vec<u8>;
    fn unpad(padded_message: Vec<u8>, consistency_checking: bool) -> Vec<u8>;
}

pub struct ANSI_X9_23 {}
impl Pad for ANSI_X9_23 {
    fn pad(message: Vec<u8>,block_size: usize) -> Vec<u8> {
        let length: usize = message.len();
        let mut padding_length = length % block_size;

        if padding_length == 0 {
            padding_length = block_size;
        }

        let mut padded_message: Vec<u8> = message.clone();
        for _i in 0..padding_length-1 {
            padded_message.push(0u8);
        }

        padded_message.push(padding_length as u8);

        padded_message
    }

    fn unpad(padded_message: Vec<u8>, consistency_checking: bool) -> Vec<u8> {
        let length: usize = padded_message.len();
        let padding_length: u8 = padded_message[length-1];

        if consistency_checking {
            let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
            let padding_is_valid: bool = padded_message[(length-(padding_length as usize))..length].to_vec().iter().all(|&x| x == 0);
            if padding_is_valid {
                return message;
            } else {
                panic!("The input message is not correctly padded");
            }
        } else {
            let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
            return message;
        }
    }
}

pub struct ISO_10126 {}
impl Pad for ISO_10126 {
    fn pad(message: Vec<u8>, block_size: usize) -> Vec<u8> {
        let length: usize = message.len();
        let mut padding_length = length % block_size;

        if padding_length == 0 {
            padding_length = block_size;
        }

        let mut padded_message: Vec<u8> = message.clone();
        for _i in 0..padding_length-1 {
            padded_message.push(rand::thread_rng().gen::<u8>());
        }

        padded_message.push(padding_length as u8);

        padded_message
    }

    fn unpad(padded_message: Vec<u8>, _consistency_checking: bool) -> Vec<u8> {
        let length: usize = padded_message.len();
        let padding_length: u8 = padded_message[length-1];

        let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
        message
        
    }
}
pub struct ISO_IEC_7816_4 {}
impl Pad for ISO_IEC_7816_4 {
    fn pad(message: Vec<u8>, block_size: usize) -> Vec<u8> {
        let length: usize = message.len();
        let mut padding_length = length % block_size;

        if padding_length == 0 {
            padding_length = block_size;
        }

        let mut padded_message: Vec<u8> = message.clone();
        padded_message.push(0x80 as u8);

        for _i in 0..padding_length-1 {
            padded_message.push(0u8);
        }

        padded_message
    }

    fn unpad(padded_message: Vec<u8>, consistency_checking: bool) -> Vec<u8> {
        let length: usize = padded_message.len();
        let mut padding_length: u8 = 0;

        // computing padding length
        for i in (0..length).rev() {
            if padded_message[i] != 0x80 {
                padding_length += 1;
            } else {
                padding_length += 1;
                break;
            }
        }
        
        if consistency_checking {
            let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
            let padding_is_valid: bool = padded_message[(length-(padding_length as usize)+1)..].to_vec().iter().all(|&x| x == 0);
            if padding_is_valid {
                return message;
            } else {
                panic!("The input message is not correctly padded");
            }
        } else {
            let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
            return message;
        }
    }
}
pub struct PKCS7 {}
impl Pad for PKCS7 {
    fn pad(message: Vec<u8>, block_size: usize) -> Vec<u8> {
        let length: usize = message.len();
        let mut padding_length = length % block_size;

        if padding_length == 0 {
            padding_length = block_size;
        }

        let mut padded_message: Vec<u8> = message.clone();
        for _i in 0..padding_length {
            padded_message.push(padding_length as u8);
        }

        padded_message
    }

    fn unpad(padded_message: Vec<u8>, consistency_checking: bool) -> Vec<u8> {
        let length: usize = padded_message.len();
        let padding_length: u8 = padded_message[length-1];

        if consistency_checking {
            let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
            let padding_is_valid: bool = padded_message[(length-(padding_length as usize))..].to_vec().iter().all(|&x| x == padding_length);
            if padding_is_valid {
                return message;
            } else {
                panic!("The input message is not correctly padded");
            }
        } else {
            let message: Vec<u8> = padded_message[0..(length-(padding_length as usize))].to_vec();
            return message;
        }
    }
}




pub struct GenericPadding {
    padding_method: Paddings
}

impl GenericPadding {
    pub fn init(pad_type: Paddings) -> Self{
        GenericPadding {padding_method: pad_type}
    }
    pub fn pad(&self, message: Vec<u8>, block_size: usize) -> Vec<u8> {
        let result: Vec<u8> = match self.padding_method { 
            Paddings::ANSI_X9_23=>ANSI_X9_23::pad(message, block_size),
            Paddings::ISO_10126=>ISO_10126::pad(message, block_size),
            Paddings::ISO_IEC_7816_4=>ISO_IEC_7816_4::pad(message, block_size),
            Paddings::PKCS7=>PKCS7::pad(message, block_size),
            _=>panic!("Method not found")        
          };

          result
    }
    pub fn unpad(&self, padded_message: Vec<u8>, consistency_checking: bool) -> Vec<u8> {
        let result: Vec<u8> = match self.padding_method { 
            Paddings::ANSI_X9_23=>ANSI_X9_23::unpad(padded_message, consistency_checking),
            Paddings::ISO_10126=>ISO_10126::unpad(padded_message, consistency_checking),
            Paddings::ISO_IEC_7816_4=>ISO_IEC_7816_4::unpad(padded_message, consistency_checking),
            Paddings::PKCS7=>PKCS7::unpad(padded_message, consistency_checking),
            _=>panic!("Method not found")        
          };

          result
    }
}