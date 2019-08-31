use std::{
    collections::{
        BTreeMap,
    },
};

use openssl::{
    symm::{
        Cipher,
        // decrypt,
        encrypt,
    },
};

use crate::{
    set_2_9::{
        pad_pkcs7,
    },
    set_2_10::{
        ecb_stream_decrypt,
    },
    set_2_11::{
        EncryptResult,
        EncryptType,
        Encrypter,
    },
};

struct ProfileCrypter {
    secret_key: Vec<u8>,
}

impl Encrypter for ProfileCrypter {
    fn pad(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8> {
        pad_pkcs7(data, 16)
    }
    
    fn encrypt(
        &self,
        data: &Vec<u8>,
    ) -> EncryptResult {
        let cipher = Cipher::aes_128_ecb();
        let new_data = self.pad(&format!(
            "email={}&uid=10&role=user",
            String::from_utf8_lossy(&data).replace("&", "").replace("=", ""),
            // percent_encode(
            //     data,
            //     CONTROLS_AND_METACHARS,
            // ),
        ).into_bytes());
        
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

impl ProfileCrypter {
    pub fn new(
    ) -> Self {
        Self {
            secret_key: Self::rand_bytes(16),
        }
    }

    fn parse(
        text: &str,
    ) -> BTreeMap<String, String> {
        text.split("&").map(|key_val| {
            let mut tup = key_val.split("=");
            (
                tup.next().map(|key| {
                    key.to_string()
                }).expect("error in fmt"),
                tup.next().map(|val| {
                    val.to_string()
                    // percent_decode(
                    //     val.as_bytes(),
                    // ).decode_utf8_lossy().to_string()
                }).expect("error in fmt"),
            )
        }).collect()
    }

    fn decrypt(
        &self,
        data: &Vec<u8>,
    ) -> BTreeMap<String, String> {
        Self::parse(
            &String::from_utf8_lossy(
                &ecb_stream_decrypt(data, &self.secret_key, None),
            ).to_string()[..],
        )
    }
}

#[allow(dead_code)]
pub fn main(
) {
    // Assume we know format is email=X&uid=10&role=Y
    // Target final block divisions as
    // email=<input 10 bytes> | <input 3 bytes>&uid=10&role= | admin<padding 11 bytes>
    // So final input must then be length 13
    let dovetail_input = "bob@gmail.com".as_bytes().to_vec();

    // Calculate admin<padding> block
    // email=<input 10 bytes> | <input admin + padding 11 bytes>
    let mut admin_input = vec!['X' as u8; 10];
    admin_input.append(&mut "admin".as_bytes().to_vec());
    admin_input.append(&mut vec![11u8].into_iter().cycle().take(11).collect());

    let crypter = ProfileCrypter::new();

    let encrypted = crypter.encrypt(
        &dovetail_input,
    ).encrypted;
    let mut output = encrypted.into_iter().take(2 * 16).collect::<Vec<u8>>();
    let mut output_end = crypter.encrypt(
        &admin_input,
    ).encrypted.into_iter().skip(1 * 16).take(1 * 16).collect::<Vec<u8>>();
    output.append(&mut output_end);

    println!("{:?}", crypter.decrypt(&output));
}
