#[cfg(test)]

use crate::{
    encoded_string::{
        EncodedString,
    },
};

#[test]
fn ascii_to_base64(
) {
    assert_eq!(
        EncodedString::ASCII("Lorem ipsum".to_string()).to_base64(),
        EncodedString::Base64("TG9yZW0gaXBzdW0=".to_string()),
    );
}

#[test]
fn ascii_to_binary(
) {
    assert_eq!(
        EncodedString::ASCII("Lorem ipsum".to_string()).to_binary(),
        EncodedString::Binary("0100110001101111011100100110010101101101001000000110100101110000011100110111010101101101".to_string()),
    );
}

#[test]
fn ascii_to_hex(
) {
    assert_eq!(
        EncodedString::ASCII("Lorem ipsum".to_string()).to_hex(),
        EncodedString::Hex("4c6f72656d20697073756d".to_string()),
    );
}

#[test]
fn base64_to_ascii(
) {
    assert_eq!(
        EncodedString::Base64("TG9yZW0gaXBzdW0=".to_string()).to_ascii(),
        EncodedString::ASCII("Lorem ipsum".to_string()),
    );
}

#[test]
fn base64_pad0_to_binary(
) {
    assert_eq!(
        EncodedString::Base64("Nhm5".to_string()).to_binary(),
        EncodedString::Binary("001101100001100110111001".to_string()),
    );
}

#[test]
fn base64_pad1_to_binary(
) {
    assert_eq!(
        EncodedString::Base64("c3U=".to_string()).to_binary(),
        EncodedString::Binary("0111001101110101".to_string()),
    );
}

#[test]
fn base64_pad2_to_binary(
) {
    assert_eq!(
        EncodedString::Base64("cw==".to_string()).to_binary(),
        EncodedString::Binary("01110011".to_string()),
    );
}

#[test]
fn base64_to_hex(
) {
    assert_eq!(
        EncodedString::Base64("Nhm5".to_string()).to_hex(),
        EncodedString::Hex("3619b9".to_string()),
    );
}

#[test]
fn binary_to_ascii(
) {
    assert_eq!(
        EncodedString::Binary("0100110001101111011100100110010101101101001000000110100101110000011100110111010101101101".to_string()).to_ascii(),
        EncodedString::ASCII("Lorem ipsum".to_string()),
    );
}

#[test]
fn binary_to_base64_pad0(
) {
    assert_eq!(
        EncodedString::Binary("001101100001100110111001".to_string()).to_base64(),
        EncodedString::Base64("Nhm5".to_string()),
    );
}

#[test]
fn binary_to_base64_pad1(
) {
    assert_eq!(
        EncodedString::Binary("0111001101110101".to_string()).to_base64(),
        EncodedString::Base64("c3U=".to_string()),
    );
}

#[test]
fn binary_to_base64_pad2(
) {
    assert_eq!(
        EncodedString::Binary("01110011".to_string()).to_base64(),
        EncodedString::Base64("cw==".to_string()),
    );
}

#[test]
fn binary_to_hex(
) {
    assert_eq!(
        EncodedString::Binary("111111110110001110101100".to_string()).to_hex(),
        EncodedString::Hex("ff63ac".to_string()),
    );
}

#[test]
fn hex_to_ascii(
) {
    assert_eq!(
        EncodedString::Hex("4c6f72656d20697073756d".to_string()).to_ascii(),
        EncodedString::ASCII("Lorem ipsum".to_string()),
    );
}

#[test]
fn hex_to_base64(
) {
    assert_eq!(
        EncodedString::Hex("3619b9".to_string()).to_base64(),
        EncodedString::Base64("Nhm5".to_string()),
    );
}

#[test]
fn hex_to_binary(
) {
    assert_eq!(
        EncodedString::Hex("ff63ac".to_string()).to_binary(),
        EncodedString::Binary("111111110110001110101100".to_string()),
    );
}
