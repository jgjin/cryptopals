pub fn pad_pkcs7(
    text: &[u8],
    block_len: usize,
) -> Vec<u8> {
    text.chunks(block_len).fold(vec![], |mut acc, chunk| {
        acc.extend(chunk.iter().map(|byte| {
            *byte
        }));

        if chunk.len() != block_len {
            let pad = block_len - chunk.len();
            acc.append(&mut vec![pad as u8; pad]);
        }

        acc
    })
}

#[allow(dead_code)]
pub fn main(
) {
    let text = "YELLOW SUBMARINE".to_string().into_bytes();
    println!("{:?}", pad_pkcs7(&text, 20));
    assert_eq!(
        pad_pkcs7(&text, 20),
        vec![
            'Y' as u8,
            'E' as u8,
            'L' as u8,
            'L' as u8,
            'O' as u8,
            'W' as u8,
            ' ' as u8,
            'S' as u8,
            'U' as u8,
            'B' as u8,
            'M' as u8,
            'A' as u8,
            'R' as u8,
            'I' as u8,
            'N' as u8,
            'E' as u8,
            4u8,
            4u8,
            4u8,
            4u8,
        ],
    )
}
