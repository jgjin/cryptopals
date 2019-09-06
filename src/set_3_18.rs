extern crate base64;
extern crate openssl;

use openssl::{
    symm::{
        Cipher,
        Crypter,
        Mode,
    }
};

pub struct CTRCrypter {
    key: Vec<u8>,
    nonce: Vec<u8>,
}

impl CTRCrypter {
    pub fn new(
    ) -> Self {
        Self {
            key: "YELLOW SUBMARINE".bytes().collect(),
            nonce: vec![0u8; 8],
        }
    }

    fn decrypt_chunk(
        &self,
        chunk: &[u8],
        counter: u64,
    ) -> Vec<u8> {
        let cipher = Cipher::aes_128_ecb();
        let mut encrypter = Crypter::new(
            cipher,
            Mode::Encrypt,
            &self.key,
            None,
        ).expect("error creating openssl crypter");

        let mut data = self.nonce.clone();
        data.append(&mut counter.to_le_bytes().to_vec());

        let mut plaintext = vec![0; data.len() + cipher.block_size()];
        encrypter.update(&data, &mut plaintext).expect("error encrypting");

        plaintext[..chunk.len()].to_vec().iter().zip(chunk.iter()).map(|(first_byte, second_byte)| {
            first_byte ^ second_byte
        }).collect()
    }

    pub fn decrypt(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8> {
        let mut counter = 0u64;
        let mut output = vec![];
        data.chunks(16).map(|chunk| {
            output.append(&mut self.decrypt_chunk(chunk, counter));
            counter += 1;
        }).last();

        output
    }
}

#[allow(dead_code)]
pub fn main(
) {
    let decrypter = CTRCrypter::new();
    let encrypted = base64::decode(
        "L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ=="
    ).expect("error converting base64");

    let decrypted = decrypter.decrypt(&encrypted);
    println!("{}", String::from_utf8_lossy(&decrypted));
}
