extern crate hamming;

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
        bytes_into_ascii,
        get_single_byte_candidates,
    },
};

#[derive(Debug)]
struct KeySizeCandidate {
    key_size: usize,
    score: f32,
}

#[allow(dead_code)]
fn eval_key_size(
    bytes: &Vec<u8>,
    key_size: usize,
) -> KeySizeCandidate {
    let len = bytes.len();
    let bytes_trunc = bytes.iter().cloned().take(
        len - (len % (key_size * 2))
    ).collect::<Vec<u8>>();

    let mut first = vec![];
    let mut second = vec![];
    bytes_trunc.chunks(key_size * 2).map(|chunk| {
        first.push(chunk.iter().map(|byte| {
            *byte
        }).take(key_size).collect::<Vec<u8>>());

        second.push(bytes.iter().map(|byte| {
            *byte
        }).skip(key_size).take(key_size).collect::<Vec<u8>>());
    }).last();

    let avg_dist = first.iter().zip(second.iter()).map(|(first_chunk, second_chunk)| {
        hamming::distance(first_chunk, second_chunk)
    }).sum::<u64>() as f32 / first.len() as f32;

    KeySizeCandidate {
        key_size: key_size,
        score: avg_dist / key_size as f32,
    }
} // outputted to key_sizes_1_6.txt

fn transpose(
    bytes: &Vec<u8>,
    num_blocks: usize,
) -> Vec<Vec<u8>> {
    let mut blocks = vec![vec![]; num_blocks];

    bytes.iter().enumerate().map(|(index, byte)| {
        blocks[index % num_blocks].push(*byte);
    }).last();

    blocks
}

#[allow(dead_code)]
pub fn main(
) {
    let bytes = base64::decode(&open_file("set_1_6.txt").join("")[..]).expect("error converting base64");

    // let mut key_size_candidates = (2..=40).map(|key_size| {
    //     eval_key_size(&bytes, key_size)
    // }).collect::<Vec<KeySizeCandidate>>();

    // key_size_candidates.sort_unstable_by(|first, second| {
    //     first.score.partial_cmp(&second.score).expect("error in sorting")
    // });

    // key_size_candidates.iter().map(|candidate| {
    //     println!("{:?}", candidate);
    // }).last();

    let key_sizes = vec![29];
    let candidates = key_sizes.into_iter().filter_map(|key_size| {
        let blocks = transpose(&bytes, key_size);
        let key_candidates = blocks.into_iter().map(|block| {
            let mut specific_candidates = get_single_byte_candidates(&block, true, false, Some(144));
            specific_candidates.sort_unstable_by(|first, second| {
                first.score.partial_cmp(&second.score).expect("error in sorting")
            });

            specific_candidates.into_iter().map(|candidate| {
                candidate.key[0]
            }).take(1).collect()
        }).collect::<Vec<Vec<u8>>>();

        if key_candidates.iter().all(|x| {
            !x.is_empty()
        }) {
            return Some(key_candidates);
        }
        return None;
    }).collect::<Vec<Vec<Vec<u8>>>>();

    candidates.into_iter().map(|possibilities| {
        let mut perm = Permutations::new(possibilities);
        while let Some(key) = perm.next() {
            let decoded = bytes_into_ascii(&xor(&bytes, &key)).expect("error converting ascii");
            println!("{}", decoded);
        }
    }).last();
}
