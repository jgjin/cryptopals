extern crate hex;

use std::{
    collections::{
        BTreeSet,
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
    let lines = open_file("set_1_8.txt");
    lines.iter().map(|line| {
        let bytes = hex::decode(line).expect("error converting hex");

        let mut blocks = BTreeSet::new();
        bytes.chunks(16).map(|chunk| {
            let block_str = chunk.iter().map(|byte| {
                *byte as char
            }).collect::<String>();
            blocks.insert(block_str);
        }).last();

        if blocks.len() < 10 {
            println!("{}: {}", line, blocks.len());
        }
    }).last();
}
