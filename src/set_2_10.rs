extern crate base64;
extern crate openssl;

use openssl::{
    symm::{
        Cipher,
        Crypter,
        Mode,
    },
};

use crate::{
    io::{
        open_file,
    },
    set_1_2::{
        xor,
    },
    set_2_9::{
        pad_pkcs7,
    },
};

pub fn ecb_stream_decrypt(
    data: &[u8],
    key: &[u8],
    iv: Option<&[u8]>,
) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut decrypter = Crypter::new(
        cipher,
        Mode::Decrypt,
        key,
        iv,
    ).expect("error creating openssl crypter");

    let mut plaintext = vec![0; data.len() + cipher.block_size()];

    let mut count = 0;
    data.chunks(16).map(|chunk| {
        count += decrypter.update(chunk, &mut plaintext[count..]).expect("error decrypting");
    }).last();

    count += decrypter.finalize(&mut plaintext[count..]).expect("error finalizing");

    plaintext.truncate(count);

    plaintext
}

pub fn ecb_decrypt(
    data: &[u8],
    key: &[u8],
    iv: Option<&[u8]>,
) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut crypter = Crypter::new(
        cipher,
        Mode::Decrypt,
        key,
        iv,
    ).expect("error creating openssl crypter");

    let mut output = vec![0; data.len() + cipher.block_size()];

    crypter.update(data, &mut output).expect("error decrypting");

    output[..cipher.block_size()].to_vec()
}

pub fn cbc_decrypt(
    data: &[u8],
    key: &[u8],
    iv: Option<&[u8]>,
) -> Vec<u8> {
    let block_size = Cipher::aes_128_cbc().block_size();

    let mut xor_op = &iv.map(|arr| {
        arr.into()
    }).unwrap_or(vec![0; block_size])[..];

    pad_pkcs7(data, block_size).chunks(block_size).fold(vec![], |mut acc, chunk| {
        let ecb_res = ecb_decrypt(chunk, key, None);
        let mut xor_res = xor(xor_op, &ecb_res[..]);

        acc.append(&mut xor_res);
        xor_op = chunk;

        acc
    })
}

#[allow(dead_code)]
pub fn main(
) {
    let data = base64::decode(&open_file("set_2_10.txt").join("")[..])
        .expect("error converting base64");
    let key = "YELLOW SUBMARINE".to_string().into_bytes();

    let decrypted = cbc_decrypt(
        &data,
        &key,
        None,
    ).into_iter().map(|byte| {
        byte as char
    }).collect::<String>();

    println!("decrypted: {:?}", decrypted);
}
