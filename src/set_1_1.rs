extern crate base64;
extern crate hex;

fn hex_to_base64(
    hex_str: &str,
) -> String {
    let binary = hex::decode(hex_str).expect("error converting hex");
    base64::encode(&binary)
}

#[allow(dead_code)]
pub fn main(
) {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert_eq!(
        hex_to_base64(hex_str),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
    );
}
