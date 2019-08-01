// need to use lib for encoding and decoding

use crate::{
    convert,
    encoded_string::{
        EncodedString,
    },
    io::{
        open_file,
    },
    permutate::{
        Permutations,
    },
    set_1_3::{
        DecodeResult,
        decode_single,
    },
};

#[derive(Debug)]
struct KeySizeCandidate {
    key_size: usize,
    norm_avg_hamming_dist: f32,
}

fn split_bytes(
    string: &str,
    key_size: usize,
) -> (Vec<String>, Vec<String>) {
    let chars = string.chars()
        .take(key_size * 2)
        .collect::<Vec<char>>();
    let first = chars.chunks(key_size * 2).filter_map(|chunk| {
        Some(chunk).filter(|chnk| {
            chnk.len() == key_size * 2
        }).map(|chnk| {
            chnk.into_iter().take(key_size).collect::<String>()
        })
    }).collect();
    let second = chars.chunks(key_size * 2).filter_map(|chunk| {
        Some(chunk).filter(|chnk| {
            chnk.len() == key_size * 2
        }).map(|chnk| {
            chnk.into_iter().skip(key_size).take(key_size).collect::<String>()
        })
    }).collect();

    (first, second)
}

fn get_key_sizes(
    string: &str,
) -> Vec<KeySizeCandidate> {
    (2..40).map(|key_size| {
        let (first, second) = split_bytes(string, key_size);
        let avg_hamming_dist = first.iter().zip(
            second.into_iter(),
        ).map(|(first_chunk, second_chunk)| {
            EncodedString::ASCII(first_chunk.to_string()).hamming_dist(
                &EncodedString::ASCII(second_chunk.to_string())
            )
        }).sum::<usize>() as f32 / first.len() as f32;

        KeySizeCandidate {
            key_size: key_size,
            norm_avg_hamming_dist: avg_hamming_dist / key_size as f32,
        }
    }).collect()
}

fn transpose(
    text: &str,
    num_blocks: usize,
) -> Vec<Vec<char>> {
    let mut blocks = vec![vec![]; num_blocks];
    text.chars().collect::<Vec<char>>().chunks(num_blocks).map(|chunk| {
        chunk.iter().enumerate().map(|(index, chr)| {
            blocks[index].push(*chr);
        }).last();
    }).last();

    blocks
}

pub fn main(
) {
    let text = EncodedString::Base64(open_file("set_1_6.txt").join("")).to_ascii();
    
    let try_key_sizes = vec![2, 5, 7];

    let mut decode_results = vec![];
    try_key_sizes.into_iter().map(|key_size| {
        let blocks = transpose(&text.inner_string()[..], key_size);
        let candidates = blocks.into_iter().map(|block| {
            let mut cand = decode_single(
                &block.into_iter().collect::<String>()[..],
            );

            cand.sort_unstable_by(|first, second| {
                first.score.partial_cmp(&second.score).expect("error in sorting")
            });


            cand.into_iter().take(3).map(|res| {
                res.key
            }).collect::<Vec<EncodedString>>()
        }).collect::<Vec<Vec<EncodedString>>>();

        let mut permuter = Permutations::new(candidates);
        while let Some(perm) = permuter.next() {
            let full_key = perm.into_iter().map(|chr| {
                chr.inner_string()
            }).collect::<String>();

            let key = EncodedString::Binary(full_key);
            let decoded = text.xor(&key).to_ascii();
            let score = decoded.freq_score();

            decode_results.push(DecodeResult {
                key: key,
                encoded: EncodedString::ASCII("Too long, omitting".to_string()),
                decoded: EncodedString::ASCII(decoded.inner_string().chars().take(144).collect::<String>()),
                score: score,
            });
        }
    }).last();

    decode_results.sort_unstable_by(|first, second| {
        first.score.partial_cmp(&second.score).expect("error in sorting")
    });

    decode_results.into_iter().take(50).map(|res| {
        println!("{:?}", res);
    }).last();
}

#[cfg(test)]

#[test]
fn test_hamming_dist(
) {
    assert_eq!(
        EncodedString::ASCII("this is a test".to_string()).hamming_dist(
            &EncodedString::ASCII("wokka wokka!!!".to_string()),
        ),
        37,
    );
}

#[test]
fn test_split_bytes_exact(
) {
    assert_eq!(
        split_bytes(
            &EncodedString::ASCII("a tester".to_string()).inner_string()[..],
            2,
        ),
        (
            vec!["a ".to_string(), "st".to_string()],
            vec!["te".to_string(), "er".to_string()],
        )
    );
}

#[test]
fn test_split_bytes_unbalanced(
) {
    assert_eq!(
        split_bytes(
            &EncodedString::ASCII(
                "a tester is here".to_string(),
            ).inner_string()[..],
            3,
        ),
        (
            vec!["a t".to_string(), "er ".to_string()],
            vec!["est".to_string(), "is ".to_string()],
        )
    );
}
