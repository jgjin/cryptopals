extern crate base64;
extern crate openssl;
extern crate rand;

use openssl::{
    symm::{
        Cipher,
        decrypt,
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
    set_2_15::{
        Decrypter,
    },
};

struct NewEncryptResult {
    encrypted: Vec<u8>,
    _iv: Vec<u8>,
}

struct NewCBCCrypter {
    data: Vec<u8>,
    secret_key: Vec<u8>,
    iv: Vec<u8>,
}

impl Decrypter for NewCBCCrypter {
    fn decrypt(
        &self,
        data: &Vec<u8>,
    ) -> bool {
        decrypt(
            Cipher::aes_128_cbc(),
            &self.secret_key,
            Some(&self.iv),
            data,
        ).is_ok()
    }
}

impl NewCBCCrypter {
    fn new(
    ) -> Self {
        let mut rng = rand::thread_rng();

        let mut lines = open_file("set_3_17.txt");
        let index = rng.gen_range(0, lines.len());
        println!(
            "orig data {:?}",
            base64::decode(&lines[index][..])
                .expect("error converting base64"),
        );

        Self {
            data: base64::decode(&lines.remove(index)[..])
                .expect("error converting base64"),
            secret_key: Self::rand_bytes(16),
            iv: Self::rand_bytes(16),
        }
    }

    fn rand_bytes(
        size: usize,
    ) -> Vec<u8> where Self: Sized {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| {
            rng.gen()
        }).collect()
    }

    fn encrypt(
        &self,
        _data: &Vec<u8>,
    ) -> NewEncryptResult {
        let cipher = Cipher::aes_128_cbc();

        NewEncryptResult {
            encrypted: encrypt(
                cipher,
                &self.secret_key,
                Some(&self.iv),
                &self.data,
            ).expect("error encrypting"),
            _iv: self.iv.clone(),
        }
    }
}

fn guess_nth_byte_padding(
    crypter: &NewCBCCrypter,
    encrypted: &Vec<u8>,
    target: usize,
    current_chunk: &Vec<u8>,
) -> u8 {
    let desired_padding = 16 - (target % 16);

    let mut new_encrypted = encrypted.clone();
    let len = new_encrypted.len();

    (0..(desired_padding)).map(|offset| {
        new_encrypted[len - 1 - offset - 16] = current_chunk[current_chunk.len() - 1 - offset] ^ new_encrypted[len - 1 - offset - 16] ^ desired_padding as u8;
    }).last();

    let candidates = (0..=255u8).filter(|guess| {
        new_encrypted[len - desired_padding - 16] = *guess;
        crypter.decrypt(&new_encrypted)
    }).collect::<Vec<u8>>();
    
    if candidates.len() > 1 {
        assert_eq!(candidates.len(), 2);
        return (candidates[0] ^ candidates[1]) ^ desired_padding as u8;
    }

    (candidates[0] ^ encrypted[len - desired_padding - 16]) ^ desired_padding as u8
}

fn padding_attack(
    crypter: &NewCBCCrypter,
) -> Vec<u8> {
    let mut encrypted = crypter.encrypt(&vec![]).encrypted;

    let mut output = vec!['X' as u8; encrypted.len()];

    let mut target = encrypted.len() - 1;
    let mut current_chunk = vec!['C' as u8; 16];

    while target >= 16 {
        let guess = guess_nth_byte_padding(
            crypter,
            &encrypted,
            target,
            &current_chunk,
        );
        current_chunk[target % 16] = guess;

        if target % 16 == 0 {
            output.splice(target..(target + 16), current_chunk.iter().cloned());
            current_chunk = vec!['C' as u8; 16];
            encrypted = encrypted[..encrypted.len() - 16].to_vec();
        }

        target -= 1;
    }

    output
}

pub fn main(
) {
    let crypter = NewCBCCrypter::new();

    let output = padding_attack(&crypter);

    println!("output {:?}", output);
}
