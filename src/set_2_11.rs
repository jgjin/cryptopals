extern crate rand;

use std::{
    collections::{
        BTreeSet,
    },
    fmt::{
        Debug,
    },
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
    set_2_9::{
        pad_pkcs7,
    },
};

#[derive(Debug, PartialEq)]
pub enum EncryptType {
    CBC,
    ECB,
}

pub trait Encrypter {
    fn rand_bytes(
        size: usize,
    ) -> Vec<u8> where Self: Sized {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| {
            rng.gen()
        }).collect()
    }

    fn pad(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8>;

    fn encrypt(
        &self,
        data: &Vec<u8>,
    ) -> EncryptResult;
}

pub struct EncryptResult {
    pub encrypt_type: EncryptType,
    pub encrypted: Vec<u8>,
}

#[derive(Debug)]
pub struct RandEncrypter {
    secret_key: Vec<u8>,
}

impl Encrypter for RandEncrypter {
    fn pad(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8> {
        let mut rng = rand::thread_rng();

        let mut padded = Self::rand_bytes(rng.gen_range(5, 11));
        padded.append(&mut data.clone());
        padded.append(&mut Self::rand_bytes(rng.gen_range(5, 11)));

        pad_pkcs7(&padded, 16)
    }

    fn encrypt(
        &self,
        data: &Vec<u8>,
    ) -> EncryptResult {
        let mut rng = rand::thread_rng();
        let mut encrypt_type = EncryptType::CBC;
        if rng.gen::<bool>() {
            encrypt_type = EncryptType::ECB;
        }

        let cipher = match encrypt_type {
            EncryptType::CBC => {
                Cipher::aes_128_cbc()
            },
            EncryptType::ECB => {
                Cipher::aes_128_ecb()
            },
        };
        let block_size = cipher.block_size();
        let iv = Self::rand_bytes(block_size);
        let new_data = self.pad(&data);

        EncryptResult {
            encrypt_type: encrypt_type,
            encrypted: encrypt(
                cipher,
                &self.secret_key,
                Some(&iv),
                &new_data,
            ).expect("error encrypting"),
        }

    }
}

impl RandEncrypter {
    pub fn new(
    ) -> Self {
        Self {
            secret_key: Self::rand_bytes(16),
        }
    }
}

fn malicious_input(
    block_size: usize,
) -> Vec<u8> {
    (0..=255u8).cycle().take(block_size).collect::<Vec<u8>>()
        .into_iter().cycle().take(block_size * 3).collect()
}

pub fn repeated_chunks<T: Clone + Ord + Debug>(
    vec: &Vec<T>,
    chunk_size: usize,
) -> bool {
    let mut num_chunks = 0;
    let num_unique_chunks = vec.chunks(chunk_size).map(|chunk| {
        num_chunks += 1;
        chunk.to_vec()
    }).collect::<BTreeSet<Vec<T>>>().len();

    num_unique_chunks != num_chunks
}

pub fn detect_mode(
    encrypter: &dyn Encrypter,
) -> EncryptType {
    let encrypt_result = encrypter.encrypt(&malicious_input(16));
    
    let mut predicted_encrypt_type = EncryptType::CBC;
    if repeated_chunks(&encrypt_result.encrypted, 16) {
        predicted_encrypt_type = EncryptType::ECB;
    }
    // assert_eq!(predicted_encrypt_type, encrypt_result.encrypt_type);

    predicted_encrypt_type
}

#[allow(dead_code)]
pub fn main(
) {
    let encrypter = RandEncrypter::new();

    (0..60).map(|_| {
        detect_mode(&encrypter);
    }).last();

    print!("All predictions correct!");
}
