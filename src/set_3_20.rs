extern crate ascii; //to remove
extern crate base64;

use crate::{
    io::{
        open_file,
    },
    permute::{
        Permutations,
    },
    set_1_2::{
        xor,
    },
    set_1_3::{
        get_single_byte_candidates,
    },
    set_3_18::{
        CTRCrypter,
    },
};

impl CTRCrypter {
    pub fn encrypt(
        &self,
        data: &Vec<u8>,
    ) -> Vec<u8> {
        self.decrypt(data)
    }
}

fn transpose(
    data: &Vec<Vec<u8>>,
) -> Vec<Vec<u8>> {
    let min_len = data.iter().map(|line| {
        line.len()
    }).min().expect("error in code");

    let mut blocks = vec![vec![]; min_len];

    data.iter().map(|line| {
        let mut line_owned = line.clone();
        line_owned.truncate(min_len);
        
        line_owned.into_iter().enumerate().map(|(index, byte)| {
            blocks[index].push(byte);
        }).last();
    }).last();

    blocks
}

fn guess_key(
    lines: &Vec<Vec<u8>>, 
) -> Vec<Vec<u8>> {
    let possibilities = transpose(&lines).into_iter().map(|block| {
        let mut specific_candidates = get_single_byte_candidates(&block, false, false, None);

        specific_candidates.sort_unstable_by(|first, second| {
            first.score.partial_cmp(&second.score).expect("error in sorting")
        });

        let mut prev_score = specific_candidates.first().map(|first_candidate| {
            first_candidate.score
        }).expect("no candidates");
        specific_candidates.into_iter().take_while(|candidate| {
            let close_enough = candidate.score - prev_score < 3f32;
            prev_score = candidate.score;
            close_enough
        }).map(|candidate| {
            candidate.key[0]
        }).collect()
    }).collect::<Vec<Vec<u8>>>();

    let mut guesses = vec![];
    let mut perm = Permutations::new(possibilities);
    while let Some(key) = perm.next() {
        guesses.push(key);
    }

    guesses
}

fn xor_lines_against(
    lines: &Vec<Vec<u8>>,
    common_xor_operand: &Vec<u8>,
) {
    println!("With key {:?}:", common_xor_operand);
    if lines.iter().all(|line| {
        // common_xor_operand (xor key) must be first arg since first arg truncates second arg
        String::from_utf8(xor(common_xor_operand, line)).is_ok()
    }) {
        lines.iter().enumerate().map(|(index, line)| {
            println!(
                "Line {}: {}",
                index,
                String::from_utf8_lossy(&xor(common_xor_operand, line)),
            );
        }).last();
    }
    else {
        println!("Not all lines valid UTF-8");
    }
}

pub fn main(
) {
    let crypter = CTRCrypter::new();

    let lines = open_file("set_3_20.txt").into_iter().map(|line| {
        let bytes = base64::decode(&line.into_bytes()).expect("error converting base64");
        crypter.encrypt(&bytes)
    }).collect::<Vec<Vec<u8>>>();

    guess_key(&lines).into_iter().map(|key| {
        xor_lines_against(&lines, &key);
    }).last();
}
