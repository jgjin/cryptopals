use crate::{
    encoded_string::{
        EncodedString,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct DecodeResult {
    pub key: EncodedString,
    pub encoded: EncodedString,
    pub decoded: EncodedString,
    pub score: f32,
}

#[allow(dead_code)]
pub fn pad_front_to(
    string: &str,
    len: usize,
    padding: char,
) -> String {
    format!(
        "{}{}",
        (0..(len - string.len())).map(|_| {
            padding
        }).collect::<String>(),
        string,
    )
}

#[allow(dead_code)]
pub fn decode_single(
    encoded_str: &str,
) -> Vec<DecodeResult> {
    let encoded = EncodedString::Hex(
        encoded_str.to_string(),
    );

    (0..=255u8).map(|chr| {
        let key = EncodedString::Binary(
            pad_front_to(&format!("{:b}", chr)[..], 8, '0'),
        );
        let decoded = encoded.xor(&key).to_ascii();
        let score = decoded.freq_score();

        DecodeResult {
            key: key,
            encoded: encoded.clone(),
            decoded: decoded,
            score: score,
        }
    }).collect()
}
