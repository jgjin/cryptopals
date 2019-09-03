pub trait Decrypter {
    fn validate_padding(
        data: &Vec<u8>,
    ) -> bool {
        let last = *data.last().expect("empty data");

        if last >= 16 {
            return false;
        }

        data.iter().rev().take(last as usize).all(|byte| {
            *byte == last
        })
    }

    fn decrypt(
        &self,
        data: &Vec<u8>,
    ) -> bool;
}

// #[allow(dead_code)]
// pub fn main(
// ) {
//     assert!(validate_padding(
//         &"ICE ICE BABY\x04\x04\x04\x04".as_bytes().to_vec()
//     ));

//     assert!(!validate_padding(
//         &"ICE ICE BABY\x05\x05\x05\x05".as_bytes().to_vec()
//     ));

//     assert!(!validate_padding(
//         &"ICE ICE BABY\x01\x02\x03\x04".as_bytes().to_vec()
//     ));
    
// }
