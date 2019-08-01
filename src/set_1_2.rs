#[cfg(test)]

use crate::{
    encoded_string::{
        EncodedString,
    },
};

#[test]
fn test(
) {
    assert_eq!(
        EncodedString::Hex("1c0111001f010100061a024b53535009181c".to_string()).xor(
            &EncodedString::Hex("686974207468652062756c6c277320657965".to_string()),
        ).to_hex(),
        EncodedString::Hex("746865206b696420646f6e277420706c6179".to_string()),
    );
}
