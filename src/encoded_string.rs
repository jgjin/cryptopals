use std::{
    collections::{
        BTreeMap,
    },
};

use crate::{
    convert::{
        *,
    },
    static_values::{
        FREQUENCIES,
    },
};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum EncodedString {
    ASCII(String),
    Base64(String),
    Binary(String),
    Hex(String),
}

#[allow(dead_code)]
impl EncodedString {
    pub fn to_binary(
        &self,
    ) -> Self {
        Self::Binary(match self {
            Self::ASCII(ascii) => {
                ascii.chars().map(|chr| {
                    ascii_char_to_binary(chr)
                }).collect()
            },
            Self::Base64(base64) => {
                base64.chars().collect::<Vec<char>>().chunks(4).map(|chunk| {
                    let mut chunk_binary: String = chunk.into_iter().map(|chr| {
                        base64_char_to_binary(*chr)
                    }).collect();

                    let chunk: String = chunk.into_iter().collect();
                    if chunk.ends_with("==") {
                        chunk_binary = chunk_binary[..8].to_string();
                    }
                    else if chunk.ends_with("=") {
                        chunk_binary = chunk_binary[..16].to_string();
                    }

                    chunk_binary
                }).collect()
            },
            Self::Binary(binary) => {
                binary.to_owned()
            },
            Self::Hex(hex) => {
                hex.chars().map(|chr| {
                    hex_char_to_binary(chr)
                }).collect()
            },
        })
    }

    pub fn to_ascii(
        &self,
    ) -> Self {
        match self {
            Self::ASCII(_) => {
                self.to_owned()
            },
            Self::Base64(_) => {
                self.to_binary().to_ascii()
            },
            Self::Binary(binary) => {
                Self::ASCII(binary.chars().collect::<Vec<char>>().chunks(
                    8,
                ).map(|chunk| {
                    if chunk.len() != 8 {
                        panic!("malformed binary to ascii");
                    }

                    let chunk: String = chunk.into_iter().collect();
                    binary_chunk_to_ascii(&chunk[..])
                }).collect())
            },
            Self::Hex(_) => {
                self.to_binary().to_ascii()
            },
        }
    }

    pub fn to_base64(
        &self,
    ) -> Self {
        match self {
            Self::ASCII(_) => {
                self.to_binary().to_base64()
            },
            Self::Base64(_) => {
                self.to_owned()
            },
            Self::Binary(binary) => {
                Self::Base64(binary.chars().collect::<Vec<char>>().chunks(
                    8,
                ).map(|chunk| {
                    if chunk.len() != 8 {
                        panic!("malformed binary to base64");
                    }

                    chunk.iter().collect()
                }).collect::<Vec<String>>().chunks(3).map(|bytes| {
                    let joined = bytes.join("");
                    let mut chunk_base64: String = joined.chars(
                    ).collect::<Vec<char>>().chunks(6).map(|chunk| {
                        binary_chunk_to_base64(
                            &format!(
                                "{}{}",
                                chunk.into_iter().collect::<String>(),
                                (0..(6-chunk.len())).into_iter().map(|_| {
                                    '0'
                                }).collect::<String>()
                            )[..]
                        )
                    }).collect();

                    if bytes.len() == 1 {
                        chunk_base64 = format!("{}==", &chunk_base64[..])
                    }
                    else if bytes.len() == 2 {
                        chunk_base64 = format!("{}=", &chunk_base64[..])
                    }

                    chunk_base64
                }).collect())
            },
            Self::Hex(_) => {
                self.to_binary().to_base64()
            },
        }
    }

    pub fn to_hex(
        &self,
    ) -> Self {
        match self {
            Self::ASCII(_) => {
                self.to_binary().to_hex()
            },
            Self::Base64(_) => {
                self.to_binary().to_hex()
            },
            Self::Binary(binary) => {
                Self::Hex(binary.chars().collect::<Vec<char>>().chunks(
                    4,
                ).map(|chunk| {
                    if chunk.len() != 4 {
                        panic!("malformed binary to hex");
                    }

                    let chunk: String = chunk.into_iter().collect();
                    binary_chunk_to_hex(&chunk[..])
                }).collect())
            },
            Self::Hex(_) => {
                self.to_owned()
            },
        }
    }

    pub fn xor(
        &self,
        other: &Self,
    ) -> Self {
        match self.to_binary() {
            Self::Binary(own_val) => {
                match other.to_binary() {
                    Self::Binary(other_val) => {
                        return Self::Binary(own_val.chars().zip(
                            other_val.chars().cycle(),
                        ).map(|(own_chr, other_chr)| {
                            Some(
                                (own_chr, other_chr),
                            ).filter(|(own_chr, other_chr)| {
                                own_chr == other_chr
                            }).map(|_| {
                                '0'
                            }).unwrap_or('1')
                        }).collect());
                    },
                    _ => {
                        panic!("error in xor");
                    },
                }
            },
            _ => {
                panic!("error in xor");
            },
        }
    }

    pub fn freq_score(
        &self,
    ) -> f32 {
        match self.to_ascii() {
            Self::ASCII(own_val) => {
                let len = own_val.len();

                let mut freqs = BTreeMap::new();
                own_val.chars().map(|chr| {
                    *freqs.entry(
                        chr.to_lowercase().next()
                            .expect("unusual character in freq_score"),
                    ).or_insert(0) += 1;
                }).last();

                return FREQUENCIES.iter().map(|(letter, freq)| {
                    (*freqs.get(letter).unwrap_or(&0) as f32 / len as f32
                     * 100f32 - freq).abs()
                }).sum();
            },
            _ => {
                panic!("error in freq_score");
            },
        }
    }

    pub fn hamming_dist(
        &self,
        other: &Self,
    ) -> usize {
        match self.to_binary() {
            Self::Binary(own_val) => {
                match other.to_binary() {
                    Self::Binary(other_val) => {
                        return own_val.chars().zip(
                            other_val.chars(),
                        ).filter(|(own_chr, other_chr)| {
                            own_chr != other_chr
                        }).count();
                    },
                    _ => {
                        panic!("error in xor");
                    },
                }
            },
            _ => {
                panic!("error in hamming_dist");
            },
        }
    }

    pub fn inner_string(
        &self,
    ) -> String {
        match self {
            Self::ASCII(ascii) => {
                return ascii.clone();
            },
            Self::Base64(base64) => {
                return base64.clone();
            },
            Self::Binary(binary) => {
                return binary.clone();
            },
            Self::Hex(hex) => {
                return hex.clone();
            },
        }
    }
}
