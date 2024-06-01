use super::symbols::{BOWL_CHAR, CONTENT_CHAR, ESC_CHAR, FILE_CHAR, VERSION_CHAR};

/// Replace bowl char sequences with escaped ones to avoid
/// problems in decoding
pub fn escape_content(content: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let bytes = content.as_slice();

    for &byte in bytes {
        match byte {
            ESC_CHAR => {
                result.push(ESC_CHAR);
                result.push(ESC_CHAR);
            }
            BOWL_CHAR => {
                result.push(ESC_CHAR);
                result.push(BOWL_CHAR);
            }
            FILE_CHAR => {
                result.push(ESC_CHAR);
                result.push(FILE_CHAR);
            }
            CONTENT_CHAR => {
                result.push(ESC_CHAR);
                result.push(CONTENT_CHAR);
            }
            VERSION_CHAR => {
                result.push(ESC_CHAR);
                result.push(VERSION_CHAR);
            }
            c => result.push(c),
        }
    }

    result
}

/// Reverts the escaping of bowl characters
pub fn unescape_content(content: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < content.len() {
        if content[i] == ESC_CHAR {
            if i + 1 < content.len() {
                result.push(content[i + 1]);
                i += 2;
            } else {
                // Handle case where ESC_CHAR is at the end
                result.push(ESC_CHAR);
                i += 1;
            }
        } else {
            result.push(content[i]);
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_escape_unescape_content() {
        // Test cases
        let test_cases = vec![
            (vec![], vec![]),                       // Empty input
            (b"hello".to_vec(), b"hello".to_vec()), // No special characters
            (
                vec![
                    b'h', b'e', b'l', b'l', b'o', b' ', ESC_CHAR, b' ', b'w', b'o', b'r', b'l',
                    b'd',
                ],
                vec![
                    b'h', b'e', b'l', b'l', b'o', b' ', ESC_CHAR, ESC_CHAR, b' ', b'w', b'o', b'r',
                    b'l', b'd',
                ],
            ), // Contains ESC_CHAR
            (
                vec![
                    b'b', b'o', b'w', b'l', b' ', BOWL_CHAR, b' ', b'c', b'h', b'a', b'r',
                ],
                vec![
                    b'b', b'o', b'w', b'l', b' ', ESC_CHAR, BOWL_CHAR, b' ', b'c', b'h', b'a', b'r',
                ],
            ), // Contains BOWL_CHAR
            (
                vec![
                    b'f', b'i', b'l', b'e', b' ', FILE_CHAR, b' ', b'c', b'h', b'a', b'r',
                ],
                vec![
                    b'f', b'i', b'l', b'e', b' ', ESC_CHAR, FILE_CHAR, b' ', b'c', b'h', b'a', b'r',
                ],
            ), // Contains FILE_CHAR
            (
                vec![
                    b'c',
                    b'o',
                    b'n',
                    b't',
                    b'e',
                    b'n',
                    b't',
                    b' ',
                    CONTENT_CHAR,
                    b' ',
                    b'c',
                    b'h',
                    b'a',
                    b'r',
                ],
                vec![
                    b'c',
                    b'o',
                    b'n',
                    b't',
                    b'e',
                    b'n',
                    b't',
                    b' ',
                    ESC_CHAR,
                    CONTENT_CHAR,
                    b' ',
                    b'c',
                    b'h',
                    b'a',
                    b'r',
                ],
            ), // Contains CONTENT_CHAR
            (
                vec![
                    b'v',
                    b'e',
                    b'r',
                    b's',
                    b'i',
                    b'o',
                    b'n',
                    b' ',
                    VERSION_CHAR,
                    b' ',
                    b'c',
                    b'h',
                    b'a',
                    b'r',
                ],
                vec![
                    b'v',
                    b'e',
                    b'r',
                    b's',
                    b'i',
                    b'o',
                    b'n',
                    b' ',
                    ESC_CHAR,
                    VERSION_CHAR,
                    b' ',
                    b'c',
                    b'h',
                    b'a',
                    b'r',
                ],
            ), // Contains VERSION_CHAR
        ];

        for (input, expected_escape) in test_cases {
            let escaped = escape_content(input.clone());
            assert_eq!(
                escaped, expected_escape,
                "Escaping failed for input: {:?}",
                input
            );

            let unescaped = unescape_content(escaped);
            assert_eq!(unescaped, input, "Unescaping failed for input: {:?}", input);
        }
    }
}
