extern crate base64;
extern crate openssl;

use openssl::{
    symm::{
        Cipher,
        decrypt,
    },
};

use crate::{
    io::{
        open_file,
    },
};

#[allow(dead_code)]
pub fn main(
) {
    let cipher = Cipher::aes_128_ecb();
    let data = base64::decode(&open_file("set_1_7.txt").join("")[..])
        .expect("error converting base64");
    let key = "YELLOW SUBMARINE".to_string().into_bytes();

    let text = decrypt(
        cipher,
        &key,
        None,
        &data,
    ).expect("error decrpyting").into_iter().map(|byte| {
        byte as char
    }).collect::<String>();

    println!("{}", text);
}
