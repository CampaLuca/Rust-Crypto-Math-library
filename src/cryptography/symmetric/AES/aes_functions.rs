use rayon::vec;

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

    let mut temp: Vec<u8> = vec![0;16];

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

    let mut temp: Vec<u8> = vec![0;16];
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
    for i in 0..current_state.len() {
        current_state[i] ^= key[i];
    }
}

pub fn sub_bytes(current_state: &mut Vec<u8>) {
    for i in 0..current_state.len() {
        current_state[i] = SBOX[current_state[i] as usize];
    }
}

pub fn inverse_sub_bytes(current_state: &mut Vec<u8>) {
    for i in 0..current_state.len() {
        current_state[i] = INV_SBOX[current_state[i] as usize];
    }
}

// private function used by the key scheduling algorithm to generate the first column of each series
fn shift_vector(vector: & mut Vec<u8>) {
    let tmp = vector[0];
    vector[0] = vector[1];
    vector[1] = vector[2];
    vector[2] = vector[3];
    vector[3] = tmp;

}





pub trait AES_KEY_EXPANDER {
    fn get_n_rounds_keys(n_rounds: usize, key: Vec<u8>) -> Vec<Vec<u8>>;
}



pub struct AES_256_KEY_EXPANDER {}
impl AES_KEY_EXPANDER for AES_256_KEY_EXPANDER {
    fn get_n_rounds_keys(n_rounds: usize, key: Vec<u8>) -> Vec<Vec<u8>> {
        let number_of_columns = (n_rounds+1)*4;
        let mut counter_columns = 0;
        
        let mut columns: Vec<Vec<u8>> = Vec::new();

        for i in 0..8 {
            let mut column: Vec<u8> = Vec::new();
            column.push(key[i*4+0]);
            column.push(key[i*4+1]);
            column.push(key[i*4+2]);
            column.push(key[i*4+3]);
            columns.push(column);
        }

        
        counter_columns = 8;
        let mut index_rcon = 1;

        for i in counter_columns..number_of_columns {
            let mut column: Vec<u8> = columns[i-1].clone();
            if i % 8 == 0 { //then use SBOX and shift
                sub_bytes(&mut column);
                shift_vector(&mut column);
                add(&mut column, &columns[i-8]);
                column[0] ^= RCON[index_rcon];
                index_rcon += 1;
            } else { // use simple byte array xor
                add(&mut column, &columns[i-8]);
            }

            columns.push(column);
        }

        assert_eq!(columns.len(), number_of_columns);

        let mut keys: Vec<Vec<u8>> = Vec::new();

        for i in num_iter::range_step(0, number_of_columns, 4) {
            let key : Vec<u8> = columns[i..i+4].concat();
            keys.push(key);
        }

        keys
    }
}


pub struct AES_192_KEY_EXPANDER {}
impl AES_KEY_EXPANDER for AES_192_KEY_EXPANDER {
    fn get_n_rounds_keys(n_rounds: usize, key: Vec<u8>) -> Vec<Vec<u8>> {
        let number_of_columns = (n_rounds+1)*4;
        let mut counter_columns = 0;
        
        let mut columns: Vec<Vec<u8>> = Vec::new();

        for i in 0..6 {
            let mut column: Vec<u8> = Vec::new();
            column.push(key[i*4+0]);
            column.push(key[i*4+1]);
            column.push(key[i*4+2]);
            column.push(key[i*4+3]);
            columns.push(column);
        }

        
        counter_columns = 6;
        let mut index_rcon = 1;

        for i in counter_columns..number_of_columns {
            let mut column: Vec<u8> = columns[i-1].clone();
            if i % 6 == 0 { //then use SBOX and shift
                sub_bytes(&mut column);
                shift_vector(&mut column);
                add(&mut column, &columns[i-6]);
                column[0] ^= RCON[index_rcon];
                index_rcon += 1;
            } else { // use simple byte array xor
                add(&mut column, &columns[i-6]);
            }

            columns.push(column);
        }

        assert_eq!(columns.len(), number_of_columns);

        let mut keys: Vec<Vec<u8>> = Vec::new();

        for i in num_iter::range_step(0, number_of_columns, 4) {
            let key : Vec<u8> = columns[i..i+4].concat();
            keys.push(key);
        }

        keys
    }
}



pub struct AES_128_KEY_EXPANDER {}
impl AES_KEY_EXPANDER for AES_128_KEY_EXPANDER {
    fn get_n_rounds_keys(n_rounds: usize, key: Vec<u8>) -> Vec<Vec<u8>> {
        let number_of_columns = (n_rounds+1)*4;
        let mut counter_columns = 0;
        
        let mut columns: Vec<Vec<u8>> = Vec::new();

        for i in 0..4 {
            let mut column: Vec<u8> = Vec::new();
            column.push(key[i*4+0]);
            column.push(key[i*4+1]);
            column.push(key[i*4+2]);
            column.push(key[i*4+3]);
            columns.push(column);
        }

        
        counter_columns = 4;
        let mut index_rcon = 1;

        for i in counter_columns..number_of_columns {
            let mut column: Vec<u8> = columns[i-1].clone();
            if i % 4 == 0 { //then use SBOX and shift
                sub_bytes(&mut column);
                shift_vector(&mut column);
                add(&mut column, &columns[i-4]);
                column[0] ^= RCON[index_rcon];
                index_rcon += 1;
            } else { // use simple byte array xor
                add(&mut column, &columns[i-4]);
            }

            columns.push(column);
        }

        assert_eq!(columns.len(), number_of_columns);

        let mut keys: Vec<Vec<u8>> = Vec::new();

        for i in num_iter::range_step(0, number_of_columns, 4) {
            let key : Vec<u8> = columns[i..i+4].concat();
            keys.push(key);
        }

        keys
    }
}


// pub struct AES_128_KEY_EXPANDER {}
// impl AES_KEY_EXPANDER for AES_128_KEY_EXPANDER {
//     fn next_key_from_expansion(current_key: &mut Vec<u8>, index: usize) {
//         let mut key: Vec<u8> = vec![0;16];
    
        
//         key[0] = RCON[index+1] ^ SBOX[current_key[13] as usize] ^ current_key[0];
//         key[1] = SBOX[current_key[14] as usize] ^ current_key[1];
//         key[2] = SBOX[current_key[15] as usize] ^ current_key[2];
//         key[3] = SBOX[current_key[12] as usize] ^ current_key[3];
    
//         for i in 4..16 {
//             key[i] = current_key[i] ^ key[i-4];
//         }
    
//         for i in 0..16 {
//             current_key[i] = key[i];
//         }
    
    
        
//     }
    
//     fn prec_key_from_expansion(current_key: &mut Vec<u8>, index: usize) {
//         let mut key: Vec<u8> = vec![0;16];
    
    
//         for i in (4..16).rev() {
//             key[i] = current_key[i] ^ current_key[i-4];
//         }
    
//         key[3] = SBOX[key[12] as usize] ^ current_key[3];
//         key[2] = SBOX[key[15] as usize] ^ current_key[2];
//         key[1] = SBOX[key[14] as usize] ^ current_key[1];
//         key[0] = SBOX[key[13] as usize] ^ current_key[0] ^ RCON[index+1];
        
//         for i in 0..16 {
//             current_key[i] = key[i];
//         }
    
//     }
    
//     fn get_ith_key_from_expansion(current_key: &mut Vec<u8>, current_index: usize, index: usize) {
//         for i in current_index..index {
//             next_key_from_expansion(current_key, i)
//         }
//     }

//     fn get_n_rounds_keys(n_rounds: usize, key: Vec<u8>) -> Vec<Vec<u8>> {
//         let mut keys: Vec<Vec<u8>> = Vec::new();

//         let mut current_key: Vec<u8> = key.clone();
      
//         keys.push(current_key.clone());

//         for i in 1..n_rounds+1 {
//             AES_128_KEY_EXPANDER::next_key_from_expansion(& mut current_key, 0);
//             keys.push(current_key.clone());
//         }

//         keys
//     }
// }



/*
INDEX is the index of the current key
*/


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



