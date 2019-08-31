extern crate hex;

pub fn xor(
    text: &[u8],
    key: &[u8],
) -> Vec<u8> {
    text.iter().zip(key.iter().cycle()).map(|(first_byte, second_byte)| {
        first_byte ^ second_byte
    }).collect()
}

#[allow(dead_code)]
pub fn main(
) {
    let first = hex::decode("1c0111001f010100061a024b53535009181c").expect("error converting hex");
    let second = hex::decode("686974207468652062756c6c277320657965").expect("error converting hex");

    let third = hex::encode(&xor(&first, &second));

    assert_eq!(
        third,
        "746865206b696420646f6e277420706c6179".to_string(),
    )
}
