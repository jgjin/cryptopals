extern crate ascii;
extern crate hex;

use std::{
    cmp::{
        min,
    },
    collections::{
        BTreeMap,
    },
};

use ascii::{
    AsciiChar,
};

use crate::{
    set_1_2::{
        xor,
    },
    static_values::{
        FREQUENCIES,
    },
};

#[derive(Debug)]
pub struct KeyCandidate {
    pub key: Vec<u8>,
    pub encoded: Option<String>,
    pub decoded: Option<String>,
    pub score: f32,
}

pub fn bytes_into_ascii(
    bytes: &Vec<u8>,
) -> Option<String> {
    bytes.iter().map(|byte| {
        AsciiChar::from(*byte).ok().filter(|ascii_chr| {
            ascii_chr.is_print() ||
                *ascii_chr == '\n' ||
                *ascii_chr == '\t'
        }).map(|ascii_chr| {
            ascii_chr.as_char()
        })
    }).collect()
}

pub fn calc_score(
    string: &str,
) -> f32 {
    let mut count = 0f32;

    let counter: BTreeMap<char, f32> = string.chars().map(|chr| {
        let ascii_chr = AsciiChar::from(chr).expect("error scoring");
        ascii_chr.to_ascii_lowercase()
    }).filter(|ascii_chr| {
        ascii_chr.is_alphabetic()
    }).fold(BTreeMap::new(), |mut acc, alpha_chr| {
        *acc.entry(alpha_chr.as_char()).or_insert(0f32) += 1f32;
        count += 1f32;
        acc
    });

    FREQUENCIES.iter().map(|(chr, freq)| {
        (freq - counter.get(chr).map(|val| {
            *val
        }).unwrap_or(0f32)).abs()
    }).sum()
}

pub fn get_single_byte_candidates(
    bytes: &Vec<u8>,
    show_decoded: bool,
    show_encoded: bool,
    display_limit: Option<usize>,
) -> Vec<KeyCandidate> {
    (0u8..=255u8).filter_map(|key| {
        let key = vec![key];
        let decoded = bytes_into_ascii(&xor(&bytes, &key));
        decoded.map(|decoded_str| {
            let mut decoded_val = None;
            if show_decoded {
                decoded_val = Some(display_limit.map(|limit| {
                    decoded_str[..min(limit, decoded_str.len())].to_string()
                }).unwrap_or(decoded_str.clone()));
            }

            let mut encoded_val = None;
            if show_encoded {
                encoded_val = Some({
                    let encoded_str = format!("{:?}", bytes);
                    display_limit.map(|limit| {
                        encoded_str[..limit].to_string()
                    }).unwrap_or(encoded_str)
                });
            }

            KeyCandidate {
                key: key,
                decoded: decoded_val,
                encoded: encoded_val,
                score: calc_score(&decoded_str[..]),
            }
        })
    }).collect()
}
