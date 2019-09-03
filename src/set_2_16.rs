extern crate openssl;

use openssl::{
    symm::{
        Cipher,
        decrypt,
        encrypt,
    },
};

use crate::{
    set_2_9::{
        pad_pkcs7,
    },
    set_2_11::{
        EncryptResult,
        EncryptType,
        Encrypter,
    },
};

struct CBCCrypter {
    secret_key: Vec<u8>,
    secret_iv: Vec<u8>,
    prefix: Vec<u8>,
    suffix: Vec<u8>,
}

impl Encrypter for CBCCrypter {
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
        let cipher = Cipher::aes_128_cbc();

        let new_data = self.pad(
            &String::from_utf8_lossy(
                data,
            ).replace("&", "").replace("=", "").into_bytes(),
        );

        EncryptResult {
            encrypt_type: EncryptType::CBC,
            encrypted: encrypt(
                cipher,
                &self.secret_key,
                Some(&self.secret_iv),
                &new_data,
            ).expect("error encrypting"),
        }
    }
}

impl CBCCrypter {
    pub fn new(
    ) -> Self {
        Self {
            secret_key: Self::rand_bytes(16),
            secret_iv: Self::rand_bytes(16),
            prefix: "comment1=cooking%20MCs;userdata=".as_bytes().to_vec(),
            suffix: ";comment2=%20like%20a%20pound%20of%20bacon".as_bytes().to_vec(),
        }
    }

    fn decrypt(
        &self,
        data: &Vec<u8>,
    ) -> String {
        String::from_utf8_lossy(
            &decrypt(
                Cipher::aes_128_cbc(),
                &self.secret_key,
                Some(&self.secret_iv),
                &data,
            ).expect("error decrypting")
        ).to_string()
    }
}

pub fn malicious_xor_op(
) -> Vec<u8> {
    let orig = ";comment2=%20lik".as_bytes().to_vec();
    let new = ";admin=true;cmt=".as_bytes().to_vec();

    orig.iter().zip(new.iter()).map(|(orig_byte, new_byte)| {
        orig_byte ^ new_byte
    }).collect()
}

pub fn malicious_xor(
    encrypted: &Vec<u8>,
) -> Vec<u8> {
    let mut result = encrypted.iter().cloned().take(2 * 16).collect::<Vec<u8>>();

    let mut next = encrypted.iter().cloned().skip(2 * 16).take(1 * 16).zip(
        malicious_xor_op().iter(),
    ).map(|(orig_byte, malicious_byte)| {
        orig_byte ^ malicious_byte
    }).collect();
    result.append(&mut next);

    let mut end = encrypted.iter().cloned().skip(3 * 16).collect();
    result.append(&mut end);
    
    result
}

#[allow(dead_code)]
pub fn main(
) {
    let crypter = CBCCrypter::new();
    let input = vec!['X' as u8; 16];
    let encrypted = crypter.encrypt(&input).encrypted;

    let decrypted = crypter.decrypt(&malicious_xor(&encrypted));
    println!("{}", decrypted);

    assert!(
        decrypted.contains(";admin=true;")
    );
}
