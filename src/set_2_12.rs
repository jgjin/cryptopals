extern crate openssl;

use openssl::{
    symm::{
        Cipher,
        encrypt,
    },
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
        detect_mode,
    },
};

struct ECBEncrypter {
    secret_key: Vec<u8>,
    suffix: Vec<u8>,
}

impl Encrypter for ECBEncrypter {
    fn pad(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8> {
        let mut padded = data.clone();
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

impl ECBEncrypter {
    pub fn new(
    ) -> Self {
        Self {
            secret_key: Self::rand_bytes(16),
            suffix: base64::decode(&open_file("set_2_12.txt").join("")[..])
                .expect("error converting base64"),
        }
    }
}

pub fn detect_block_size(
    encrypter: &dyn Encrypter,
) -> usize {
    let mut input = vec![0];
    let mut encrypt_result = encrypter.encrypt(&input);
    let orig_len = encrypt_result.encrypted.len();
    while encrypt_result.encrypted.len() == orig_len {
        input.push(0);
        encrypt_result = encrypter.encrypt(&input);
    }

    encrypt_result.encrypted.len() - orig_len
}

fn guess_nth_byte(
    nth: usize,
    known_so_far: &Vec<u8>,
    encrypter: &dyn Encrypter,
) -> Option<u8> {
    assert_eq!(nth, known_so_far.len());

    let block_size = detect_block_size(encrypter);
    let mut input = vec![0; block_size - (nth % block_size) - 1];
    let bytes_to_compare = (nth / block_size + 1) * block_size;
    let mut comparable = encrypter.encrypt(&input).encrypted;
    if bytes_to_compare + block_size == comparable.len() {
        return None;
    }
    comparable = comparable[..bytes_to_compare].to_vec();

    input.append(&mut known_so_far.clone());
    let mut guess = 0;
    input.push(guess);
    while encrypter.encrypt(&input).encrypted[..bytes_to_compare].to_vec() != comparable {
        if guess == 255u8 {
            panic!("error in code");
        }

        guess += 1;
        *input.last_mut().expect("error in code") = guess;
    }

    Some(guess)
}

fn guess_bytes(
    encrypter: &dyn Encrypter,
) -> Vec<u8> {
    let mut nth = 0;
    let mut known_so_far = vec![];
    
    while let Some(byte) = guess_nth_byte(nth, &known_so_far, encrypter) {
        nth += 1;
        known_so_far.push(byte);
    }

    known_so_far
}

#[allow(dead_code)]
pub fn main(
) {
    let encrypter = ECBEncrypter::new();

    assert_eq!(detect_block_size(&encrypter), 16);
    assert_eq!(detect_mode(&encrypter), EncryptType::ECB);

    println!(
        "{}",
        String::from_utf8(guess_bytes(&encrypter)).expect("invalid utf-8"),
    );
}
