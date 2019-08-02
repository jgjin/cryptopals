extern crate hex;

use crate::{
    set_1_2::{
        xor,
    },
};

#[allow(dead_code)]
pub fn main(
) {
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string().into_bytes();
    let key = "ICE".to_string().into_bytes();

    let encoded = hex::encode(xor(&text, &key));

    assert_eq!(
        encoded,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_string(),
    );
}
