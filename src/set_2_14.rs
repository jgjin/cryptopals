extern crate rand;

use std::{
    collections::{
        BTreeSet,
    }
};

use openssl::{
    symm::{
        Cipher,
        encrypt,
    },
};
use rand::{
    Rng,
};

use crate::{
    io::{
        open_file,
    },
    set_2_9::{
        pad_pkcs7,
    },
    set_2_11::{
        EncryptResult,
        EncryptType,
        Encrypter,
        repeated_chunks,
    },
    set_2_12::{
        detect_block_size,
    },
};

struct NewECBEncrypter {
    secret_key: Vec<u8>,
    prefix: Vec<u8>,
    suffix: Vec<u8>,
}

impl Encrypter for NewECBEncrypter {
    fn pad(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8> {
        let mut padded = self.prefix.clone();
        padded.append(&mut data.clone());
        padded.append(&mut self.suffix.clone());

        pad_pkcs7(&padded, 16)
    }

    fn encrypt(
        &self,
        data: &Vec<u8>,
    ) -> EncryptResult {
        let cipher = Cipher::aes_128_ecb();

        let new_data = self.pad(&data);

        EncryptResult {
            encrypt_type: EncryptType::ECB,
            encrypted: encrypt(
                cipher,
                &self.secret_key,
                None,
                &new_data,
            ).expect("error encrypting"),
        }
    }
}

impl NewECBEncrypter {
    pub fn new(
    ) -> Self {
        Self {
            secret_key: Self::rand_bytes(16),
            prefix: Self::rand_bytes(
                rand::thread_rng().gen_range(0, 16),
            ),
            suffix: base64::decode(&open_file("set_2_12.txt").join("")[..])
                .expect("error converting base64"),
        }
    }
}

fn calc_bytes_before(
    encrypter: &dyn Encrypter,
) -> usize {
    let repeated_input = "YELLOW_SUBMARINEYELLOW_SUBMARINE";
    let mut prefix = String::new();

    let block_size = detect_block_size(encrypter);
    while !repeated_chunks(
        &encrypter.encrypt(
            &format!("{}{}", prefix, repeated_input).into_bytes(),
        ).encrypted,
        block_size,
    ) {
        prefix.push('P');
    }

    let mut duplicate_index = 0;
    let mut chunks = BTreeSet::new();
    encrypter.encrypt(
        &format!("{}{}", prefix, repeated_input).into_bytes(),
    ).encrypted.chunks(block_size).take_while(|chunk| {
        if chunks.contains(chunk) {
            return false;
        }

        chunks.insert(chunk.clone());
        true
    }).map(|_| {
        duplicate_index += 1;
    }).last();

    if prefix.is_empty() {
        return (duplicate_index - 1) * block_size
    }

    (duplicate_index - 2) * block_size + block_size - prefix.len()
}

fn guess_nth_byte_after(
    nth: usize,
    known_so_far: &Vec<u8>,
    encrypter: &dyn Encrypter,
    prefix: &Vec<u8>,
    chunk_offset: usize,
) -> Option<u8> {
    let block_size = detect_block_size(encrypter);
    let mut input = vec!['I' as u8; block_size - (nth % block_size) - 1];
    let chunk_to_cmp = nth / block_size + chunk_offset;

    let mut true_input = vec![];
    true_input.append(&mut prefix.clone());
    true_input.append(&mut input);

    let mut comparable = encrypter.encrypt(&true_input).encrypted;
    if (chunk_to_cmp + 1) * block_size > comparable.len() {
        return None;
    }
    comparable = comparable.chunks(block_size)
        .skip(chunk_to_cmp).next()
        .expect("error in code").to_vec();

    true_input.append(&mut known_so_far.clone());
    let mut guess = 0;
    true_input.push(guess);
    while encrypter.encrypt(&true_input).encrypted.chunks(block_size)
        .skip(chunk_to_cmp).next()
        .expect("error in code").to_vec() != comparable {
            if guess == 255u8 {
                return None;
            }

            guess += 1;
            *true_input.last_mut().expect("error in code") = guess;
    }

    Some(guess)
}

fn get_bytes_after(
    encrypter: &dyn Encrypter,
) -> Vec<u8> {
    let bytes_before = calc_bytes_before(encrypter);
    let block_size = detect_block_size(encrypter);
    let prefix = vec!['P' as u8; block_size - (bytes_before % block_size)];
    let chunk_offset = (bytes_before + prefix.len()) / block_size;

    let mut nth = 0;
    let mut known_so_far = vec![];
    while let Some(byte) = guess_nth_byte_after(
        nth,
        &known_so_far,
        encrypter,
        &prefix,
        chunk_offset,
    ) {
        nth += 1;
        known_so_far.push(byte);
    }

    known_so_far
}

pub fn main(
) {
    let encrypter = NewECBEncrypter::new();

    let bytes_after = get_bytes_after(&encrypter);

    println!("{:?}", String::from_utf8_lossy(&bytes_after));
}
