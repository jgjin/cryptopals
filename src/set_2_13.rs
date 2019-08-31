extern crate percent_encoding;

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
// use percent_encoding::{
//     AsciiSet,
//     CONTROLS,
//     percent_decode,
//     percent_encode,
// };

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

// // Assume we know format, so function not needed
// fn calc_bytes_before(
//     encrypter: &dyn Encrypter,
// ) -> usize {
//     let repeated_input = "YELLOW_SUBMARINEYELLOW_SUBMARINE";
//     let mut prefix = String::new();

//     let block_size = detect_block_size(encrypter);
//     while !repeated_chunks(
//         &encrypter.encrypt(
//             &format!("{}{}", prefix, repeated_input).into_bytes(),
//         ).encrypted,
//         block_size,
//     ) {
//         prefix.push('P');
//     }

//     let mut duplicate_index = 0;
//     let mut chunks = BTreeSet::new();
//     encrypter.encrypt(
//         &format!("{}{}", prefix, repeated_input).into_bytes(),
//     ).encrypted.chunks(block_size).take_while(|chunk| {
//         if chunks.contains(chunk) {
//             return false;
//         }
        
//         chunks.insert(chunk.clone());
//         true
//     }).map(|_| {
//         duplicate_index += 1;
//     }).last();

//     if prefix.is_empty() {
//         return (duplicate_index - 1) * block_size
//     }
    
//     (duplicate_index - 2) * block_size + block_size - prefix.len()
// }

// // Assume we know format, so function not needed
// // Function does not work when & and = are removed or replaced with percent encoded equivalents
// // because when trying to guess character after & or =, cannot give & or = in user input
// // without that & or = being removed or replaced
// fn guess_nth_byte_after(
//     nth: usize,
//     known_so_far: &Vec<u8>,
//     encrypter: &dyn Encrypter,
//     prefix: &Vec<u8>,
//     chunk_offset: usize,
// ) -> Option<u8> {
//     let block_size = detect_block_size(encrypter);
//     let mut input = vec!['I' as u8; block_size - (nth % block_size) - 1];
//     let chunk_to_cmp = nth / block_size + chunk_offset;

//     let mut true_input = vec![];
//     true_input.append(&mut prefix.clone());
//     true_input.append(&mut input);

//     let mut comparable = encrypter.encrypt(&true_input).encrypted;
//     if (chunk_to_cmp + 1) * block_size > comparable.len() {
//         return None;
//     }
//     comparable = comparable.chunks(block_size)
//         .skip(chunk_to_cmp).next()
//         .expect("error in code").to_vec();

//     true_input.append(&mut known_so_far.clone());
//     let mut guess = 0;
//     true_input.push(guess);
//     while encrypter.encrypt(&true_input).encrypted.chunks(block_size)
//         .skip(chunk_to_cmp).next()
//         .expect("error in code").to_vec() != comparable {
//             if guess == 255u8 {
//                 // because & and = are eaten by encoder
//                 println!("no match among all 256 byte values");
//                 return None;
//             }

//             guess += 1;
//             *true_input.last_mut().expect("error in code") = guess;
//     }

//     Some(guess)
// }

// fn get_bytes_after(
//     encrypter: &dyn Encrypter,
// ) -> Vec<u8> {
//     let bytes_before = calc_bytes_before(encrypter);
//     let block_size = detect_block_size(encrypter);
//     let prefix = vec!['P' as u8; block_size - (bytes_before % block_size)];
//     let chunk_offset = (bytes_before + prefix.len()) / block_size;

//     let mut nth = 0;
//     let mut known_so_far = vec![];
    
//     while let Some(byte) = guess_nth_byte_after(
//         nth,
//         &known_so_far,
//         encrypter,
//         &prefix,
//         chunk_offset,
//     ) {
//         nth += 1;
//         known_so_far.push(byte);
//     }

//     known_so_far
// }

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
