use crate::{
    io::{
        open_file,
    },
    set_1_3::{
        decode_single,
    },
};

#[allow(dead_code)]
pub fn main(
) {
    let lines = open_file("set_1_4.txt");
    let mut decode_results = lines.into_iter().fold(vec![], |mut acc, line| {
        let mut single_results = decode_single(&line[..]);
        acc.append(&mut single_results);

        acc
    });

    decode_results.sort_unstable_by(|first, second| {
        first.score.partial_cmp(&second.score).expect("error in sorting")
    });

    decode_results.into_iter().take(50).map(|res| {
        println!("{:?}", res);
    }).last();
}
