use crate::cryptography::symmetric::AES::data::*;

pub fn shift_rows(current_state: &mut Vec<u8>) {
    // the length should be 16
    if current_state.len() != 16 {
        panic!("Block length should be 16");
    }

    // first row
    // nothing to do

    // second row
    //save v[1] in temp, shift the others
    let mut temp: u8 = current_state[1];
    current_state[1] = current_state[5];
    current_state[5] = current_state[9];
    current_state[9] = current_state[13];
    current_state[13] = temp;

    // third row
    temp = current_state[6];
    current_state[6] = current_state[14];
    current_state[14] = temp;
    temp = current_state[2];
    current_state[2] = current_state[10];
    current_state[10] = temp;

    // forth row
    temp = current_state[15];
    current_state[15] = current_state[11];
    current_state[11] = current_state[7];
    current_state[7] = current_state[3];
    current_state[3] = temp;


}


pub fn inverse_shift_rows(current_state: &mut Vec<u8>) {
    // the length should be 16
    if current_state.len() != 16 {
        panic!("Block length should be 16");
    }

    // first row
    // nothing to do

    // second row
    //save v[1] in temp, shift the others
    let mut temp: u8 = current_state[13];
    current_state[13] = current_state[9];
    current_state[9] = current_state[5];
    current_state[5] = current_state[1];
    current_state[1] = temp;

    // third row
    temp = current_state[6];
    current_state[6] = current_state[14];
    current_state[14] = temp;
    temp = current_state[2];
    current_state[2] = current_state[10];
    current_state[10] = temp;

    // forth row
    temp = current_state[3];
    current_state[3] = current_state[7];
    current_state[7] = current_state[11];
    current_state[11] = current_state[15];
    current_state[15] = temp;

}


pub fn mix_columns(current_state: &mut Vec<u8>) {
    if current_state.len() != 16 {
        panic!("Block length should be 16");
    }

    let mut temp: Vec<u8> = vec![0, 16];

    for i in 0..4 {
        temp[i*4+0] = MUL_2[current_state[(4*i)+0] as usize ] ^ MUL_3[current_state[(4*i)+1] as usize ] ^ current_state[(4*i)+2] ^ current_state[(4*i)+3];
        temp[i*4+1] = current_state[(4*i)+0] ^ MUL_2[current_state[(4*i)+1] as usize ] ^ MUL_3[current_state[(4*i)+2] as usize ] ^ current_state[(4*i)+3];
        temp[i*4+2] = current_state[(4*i)+0] ^ current_state[(4*i)+1] ^ MUL_2[current_state[(4*i)+2] as usize ] ^ MUL_3[current_state[(4*i)+3] as usize ];
        temp[i*4+3] = MUL_3[current_state[(4*i)+0] as usize ] ^ current_state[(4*i)+1] ^ current_state[(4*i)+2] ^ MUL_2[current_state[(4*i)+3] as usize ];
    }

    for i in 0..16 {
        current_state[i] = temp[i];
    }
}


pub fn inverse_mix_columns(current_state: &mut Vec<u8>) {
    if current_state.len() != 16 {
        panic!("Block length should be 16");
    }

    let mut temp: Vec<u8> = vec![0,16];
    for i in 0..4 {
        temp[i*4+0] = MUL_14[current_state[(4*i)+0] as usize ] ^ MUL_11[current_state[(4*i)+1] as usize ] ^ MUL_13[current_state[(4*i)+2] as usize ] ^ MUL_9[current_state[(4*i)+3] as usize ];
        temp[i*4+1] = MUL_9[current_state[(4*i)+0] as usize ] ^ MUL_14[current_state[(4*i)+1] as usize ] ^ MUL_11[current_state[(4*i)+2] as usize ] ^ MUL_13[current_state[(4*i)+3] as usize ];
        temp[i*4+2] = MUL_13[current_state[(4*i)+0] as usize ] ^ MUL_9[current_state[(4*i)+1] as usize ] ^ MUL_14[current_state[(4*i)+2] as usize ] ^ MUL_11[current_state[(4*i)+3] as usize ];
        temp[i*4+3] = MUL_11[current_state[(4*i)+0] as usize ] ^ MUL_13[current_state[(4*i)+1] as usize ] ^ MUL_9[current_state[(4*i)+2] as usize ] ^ MUL_14[current_state[(4*i)+3] as usize ];
    }

    for i in 0..16 {
        current_state[i] = temp[i];
    }
}


pub fn add(current_state: &mut Vec<u8>, key: &Vec<u8>) {
    for i in 0..16 {
        current_state[i] ^= key[i];
    }
}

pub fn sub_bytes(current_state: &mut Vec<u8>) {
    for i in 0..16 {
        current_state[i] = SBOX[current_state[i] as usize];
    }
}

pub fn inverse_sub_bytes(current_state: &mut Vec<u8>) {
    for i in 0..16 {
        current_state[i] = INV_SBOX[current_state[i] as usize];
    }
}

/*
INDEX is the index of the current key
*/
pub fn next_key_from_expansion(current_key: &mut Vec<u8>, index: usize) {
    let mut key: Vec<u8> = vec![0,16];

    key[0] = RCON[index+1] ^ SBOX[current_key[13] as usize] ^ current_key[0];
    key[1] = SBOX[current_key[14] as usize] ^ current_key[1];
    key[2] = SBOX[current_key[15] as usize] ^ current_key[2];
    key[3] = SBOX[current_key[12] as usize] ^ current_key[3];

    for i in 4..16 {
        key[i] = current_key[i] ^ key[i-4];
    }

    for i in 0..16 {
        current_key[i] = key[i];
    }
}

pub fn prec_key_from_expansion(current_key: &mut Vec<u8>, index: usize) {
    let mut key: Vec<u8> = vec![0,16];

    for i in (4..16).rev() {
        key[i] = current_key[i] ^ current_key[i-4];
    }

    key[3] = SBOX[current_key[12] as usize] ^ current_key[3];
    key[2] = SBOX[current_key[15] as usize] ^ current_key[2];
    key[1] = SBOX[current_key[14] as usize] ^ current_key[1];
    key[0] = SBOX[current_key[13] as usize] ^ current_key[0] ^ RCON[index];
    
    for i in 0..16 {
        current_key[i] = key[i];
    }

}

pub fn get_ith_key_from_expansion(current_key: &mut Vec<u8>, current_index: usize, index: usize) {
    for i in current_index..index {
        next_key_from_expansion(current_key, i)
    }
}

pub fn forward_round(current_state: &mut Vec<u8>) {
    sub_bytes(current_state);
    shift_rows(current_state);
    mix_columns(current_state);
}

pub fn backward_round(current_state: &mut Vec<u8>) {
    inverse_mix_columns(current_state);
    inverse_shift_rows(current_state);
    inverse_sub_bytes(current_state);
}



