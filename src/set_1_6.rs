use crate::{
    encoded_string::{
        EncodedString,
    },
    io::{
        open_file,
    },
    permutations::{
        Permutations,
    },
    set_1_3::{
        pad_front_to,
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

pub fn main(
) {
    let try_key_sizes = vec![2, 5, 7];

    try_key_sizes.into_iter().map(|key_size| {
        let mut permutations = Permutations::new(0..255u8, key_size);
        while let Some(key_vec) = permutations.next() {
            let key_binary = key_vec.into_iter().map(|val| {
                pad_front_to(&format!("{:b}", val)[..], 8, '0')
            }).collect::<Vec<String>>().join("");
            println!("{}", key_binary);
        }
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
