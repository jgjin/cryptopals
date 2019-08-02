extern crate hex;

use crate::{
    io::{
        open_file,
    },
    set_1_3::{
        get_single_byte_candidates,
    },
};

#[allow(dead_code)]
pub fn main(
) {
    let mut candidates = open_file("set_1_4.txt").into_iter().fold(vec![], |mut acc, line| {
        let bytes = hex::decode(&line[..]).expect("error converting hex");
        let mut next_candidates = get_single_byte_candidates(&bytes, true, true, None);

        acc.append(&mut next_candidates);

        acc
    });

    candidates.sort_unstable_by(|first, second| {
        first.score.partial_cmp(&second.score).expect("error in sorting")
    });

    candidates.iter().map(|candidate| {
        println!("{:?}", candidate);
    }).last();
}
