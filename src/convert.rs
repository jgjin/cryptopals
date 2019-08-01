#[allow(dead_code)]
pub fn ascii_char_to_binary(
    ascii_char: char,
) -> &'static str {
    match ascii_char {
        '\n' => "00001010",
        ' ' => "00100000",
        '!' => "00100001",
        '"' => "00100010",
        '#' => "00100011",
        '$' => "00100100",
        '%' => "00100101",
        '&' => "00100110",
        '\'' => "00100111",
        '(' => "00101000",
        ')' => "00101001",
        '*' => "00101010",
        '+' => "00101011",
        ',' => "00101100",
        '-' => "00101101",
        '.' => "00101110",
        '/' => "00101111",
        '0' => "00110000",
        '1' => "00110001",
        '2' => "00110010",
        '3' => "00110011",
        '4' => "00110100",
        '5' => "00110101",
        '6' => "00110110",
        '7' => "00110111",
        '8' => "00111000",
        '9' => "00111001",
        ':' => "00111010",
        ';' => "00111011",
        '<' => "00111100",
        '=' => "00111101",
        '>' => "00111110",
        '?' => "00111111",
        '@' => "01000000",
        'A' => "01000001",
        'B' => "01000010",
        'C' => "01000011",
        'D' => "01000100",
        'E' => "01000101",
        'F' => "01000110",
        'G' => "01000111",
        'H' => "01001000",
        'I' => "01001001",
        'J' => "01001010",
        'K' => "01001011",
        'L' => "01001100",
        'M' => "01001101",
        'N' => "01001110",
        'O' => "01001111",
        'P' => "01010000",
        'Q' => "01010001",
        'R' => "01010010",
        'S' => "01010011",
        'T' => "01010100",
        'U' => "01010101",
        'V' => "01010110",
        'W' => "01010111",
        'X' => "01011000",
        'Y' => "01011001",
        'Z' => "01011010",
        '[' => "01011011",
        '\\' => "01011100",
        ']' => "01011101",
        '^' => "01011110",
        '_' => "01011111",
        '`' => "01100000",
        'a' => "01100001",
        'b' => "01100010",
        'c' => "01100011",
        'd' => "01100100",
        'e' => "01100101",
        'f' => "01100110",
        'g' => "01100111",
        'h' => "01101000",
        'i' => "01101001",
        'j' => "01101010",
        'k' => "01101011",
        'l' => "01101100",
        'm' => "01101101",
        'n' => "01101110",
        'o' => "01101111",
        'p' => "01110000",
        'q' => "01110001",
        'r' => "01110010",
        's' => "01110011",
        't' => "01110100",
        'u' => "01110101",
        'v' => "01110110",
        'w' => "01110111",
        'x' => "01111000",
        'y' => "01111001",
        'z' => "01111010",
        '{' => "01111011",
        '|' => "01111100",
        '}' => "01111101",
        '~' => "01111110",
        _ => {
            panic!("unexpected ascii to binary");
        }
    }
}

#[allow(dead_code)]
pub fn base64_char_to_binary(
    base64_char: char,
) -> &'static str {
    match base64_char {
        'A' => "000000",
        'B' => "000001",
        'C' => "000010",
        'D' => "000011",
        'E' => "000100",
        'F' => "000101",
        'G' => "000110",
        'H' => "000111",
        'I' => "001000",
        'J' => "001001",
        'K' => "001010",
        'L' => "001011",
        'M' => "001100",
        'N' => "001101",
        'O' => "001110",
        'P' => "001111",
        'Q' => "010000",
        'R' => "010001",
        'S' => "010010",
        'T' => "010011",
        'U' => "010100",
        'V' => "010101",
        'W' => "010110",
        'X' => "010111",
        'Y' => "011000",
        'Z' => "011001",
        'a' => "011010",
        'b' => "011011",
        'c' => "011100",
        'd' => "011101",
        'e' => "011110",
        'f' => "011111",
        'g' => "100000",
        'h' => "100001",
        'i' => "100010",
        'j' => "100011",
        'k' => "100100",
        'l' => "100101",
        'm' => "100110",
        'n' => "100111",
        'o' => "101000",
        'p' => "101001",
        'q' => "101010",
        'r' => "101011",
        's' => "101100",
        't' => "101101",
        'u' => "101110",
        'v' => "101111",
        'w' => "110000",
        'x' => "110001",
        'y' => "110010",
        'z' => "110011",
        '0' => "110100",
        '1' => "110101",
        '2' => "110110",
        '3' => "110111",
        '4' => "111000",
        '5' => "111001",
        '6' => "111010",
        '7' => "111011",
        '8' => "111100",
        '9' => "111101",
        '+' => "111110",
        '/' => "111111",
        '=' => "______",
        _ => {
            panic!("invalid base64 to binary");
        },
    }
}

#[allow(dead_code)]
pub fn binary_chunk_to_ascii(
    binary_chunk: &str,
) -> char {
    match binary_chunk {
        "00100000" => ' ',
        "00100001" => '!',
        "00100010" => '"',
        "00100011" => '#',
        "00100100" => '$',
        "00100101" => '%',
        "00100110" => '&',
        "00100111" => '\'',
        "00101000" => '(',
        "00101001" => ')',
        "00101010" => '*',
        "00101011" => '+',
        "00101100" => ',',
        "00101101" => '-',
        "00101110" => '.',
        "00101111" => '/',
        "00110000" => '0',
        "00110001" => '1',
        "00110010" => '2',
        "00110011" => '3',
        "00110100" => '4',
        "00110101" => '5',
        "00110110" => '6',
        "00110111" => '7',
        "00111000" => '8',
        "00111001" => '9',
        "00111010" => ':',
        "00111011" => ';',
        "00111100" => '<',
        "00111101" => '=',
        "00111110" => '>',
        "00111111" => '?',
        "01000000" => '@',
        "01000001" => 'A',
        "01000010" => 'B',
        "01000011" => 'C',
        "01000100" => 'D',
        "01000101" => 'E',
        "01000110" => 'F',
        "01000111" => 'G',
        "01001000" => 'H',
        "01001001" => 'I',
        "01001010" => 'J',
        "01001011" => 'K',
        "01001100" => 'L',
        "01001101" => 'M',
        "01001110" => 'N',
        "01001111" => 'O',
        "01010000" => 'P',
        "01010001" => 'Q',
        "01010010" => 'R',
        "01010011" => 'S',
        "01010100" => 'T',
        "01010101" => 'U',
        "01010110" => 'V',
        "01010111" => 'W',
        "01011000" => 'X',
        "01011001" => 'Y',
        "01011010" => 'Z',
        "01011011" => '[',
        "01011100" => '\\',
        "01011101" => ']',
        "01011110" => '^',
        "01011111" => '_',
        "01100000" => '`',
        "01100001" => 'a',
        "01100010" => 'b',
        "01100011" => 'c',
        "01100100" => 'd',
        "01100101" => 'e',
        "01100110" => 'f',
        "01100111" => 'g',
        "01101000" => 'h',
        "01101001" => 'i',
        "01101010" => 'j',
        "01101011" => 'k',
        "01101100" => 'l',
        "01101101" => 'm',
        "01101110" => 'n',
        "01101111" => 'o',
        "01110000" => 'p',
        "01110001" => 'q',
        "01110010" => 'r',
        "01110011" => 's',
        "01110100" => 't',
        "01110101" => 'u',
        "01110110" => 'v',
        "01110111" => 'w',
        "01111000" => 'x',
        "01111001" => 'y',
        "01111010" => 'z',
        "01111011" => '{',
        "01111100" => '|',
        "01111101" => '}',
        "01111110" => '~',
        _ => ' '
    }
}

#[allow(dead_code)]
pub fn binary_chunk_to_base64(
    binary_chunk: &str,
) -> char {
    match binary_chunk {
        "000000" => 'A',
        "000001" => 'B',
        "000010" => 'C',
        "000011" => 'D',
        "000100" => 'E',
        "000101" => 'F',
        "000110" => 'G',
        "000111" => 'H',
        "001000" => 'I',
        "001001" => 'J',
        "001010" => 'K',
        "001011" => 'L',
        "001100" => 'M',
        "001101" => 'N',
        "001110" => 'O',
        "001111" => 'P',
        "010000" => 'Q',
        "010001" => 'R',
        "010010" => 'S',
        "010011" => 'T',
        "010100" => 'U',
        "010101" => 'V',
        "010110" => 'W',
        "010111" => 'X',
        "011000" => 'Y',
        "011001" => 'Z',
        "011010" => 'a',
        "011011" => 'b',
        "011100" => 'c',
        "011101" => 'd',
        "011110" => 'e',
        "011111" => 'f',
        "100000" => 'g',
        "100001" => 'h',
        "100010" => 'i',
        "100011" => 'j',
        "100100" => 'k',
        "100101" => 'l',
        "100110" => 'm',
        "100111" => 'n',
        "101000" => 'o',
        "101001" => 'p',
        "101010" => 'q',
        "101011" => 'r',
        "101100" => 's',
        "101101" => 't',
        "101110" => 'u',
        "101111" => 'v',
        "110000" => 'w',
        "110001" => 'x',
        "110010" => 'y',
        "110011" => 'z',
        "110100" => '0',
        "110101" => '1',
        "110110" => '2',
        "110111" => '3',
        "111000" => '4',
        "111001" => '5',
        "111010" => '6',
        "111011" => '7',
        "111100" => '8',
        "111101" => '9',
        "111110" => '+',
        "111111" => '/',
        _ => {
            panic!("invalid binary chunk to base64");
        },
    }
}

#[allow(dead_code)]
pub fn binary_chunk_to_hex(
    binary_chunk: &str,
) -> char {
    match binary_chunk {
        "0000" => '0',
        "0001" => '1',
        "0010" => '2',
        "0011" => '3',
        "0100" => '4',
        "0101" => '5',
        "0110" => '6',
        "0111" => '7',
        "1000" => '8',
        "1001" => '9',
        "1010" => 'a',
        "1011" => 'b',
        "1100" => 'c',
        "1101" => 'd',
        "1110" => 'e',
        "1111" => 'f',
        _ => {
            panic!("invalid binary chunk to hex");
        },
    }
}

#[allow(dead_code)]
pub fn hex_char_to_binary(
    hex_char: char,
) -> &'static str {
    match hex_char {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => {
            panic!("invalid hex to binary");
        },
    }
}
