//! This file contains information about the encoding of Bowl files
//!
//! The encoding is as follows (capitalized means constant mapped to u8):
//!
//! BOWLFILE: BOWL + VERSION + F1 + F2 + F3 + ...
//! FILE IN BOWLFILE: BOWL + FILE + <filename> + CONTENT + <escaped file contents>.
//!
//! *NOTE: This module will likely be broken into multiple later on*
//! For example, I want to create separate files for versions of the BowlFile

use std::{fs, path::PathBuf};

const ESC_CHAR: u8 = 0xFF;
const BOWL_CHAR: u8 = 0x9A;
const FILE_CHAR: u8 = 0x9C;
const CONTENT_CHAR: u8 = 0x9E;
const VERSION_CHAR: u8 = 0xA0;

const CURRENT_VERSION: &str = "0.0.1";

/// Represents the parsed version of a bowl template
#[derive(Debug, Clone)]
pub struct BowlFile {
    pub version: String,
    pub files: Vec<FileContent>,
}

#[derive(Clone)]
enum DecodeState {
    Reading,
    ReadingVersionNum(i32),
    ReadingFileName,
    ReadingFileContent,
}

impl BowlFile {
    pub fn new(files: Vec<FileContent>) -> Self {
        Self {
            version: CURRENT_VERSION.to_owned(),
            files,
        }
    }

    /// Parse a BowlFile
    /// NOTE: This may be a good use case for Nom
    pub fn decode(raw: Vec<u8>) -> Result<Self, &'static str> {
        let mut version = String::new();
        let mut files = Vec::new();
        let mut bytes = raw.as_slice();
        let mut reading_state = DecodeState::Reading;
        let mut file_path = String::new();
        let mut file_content = Vec::new();
        loop {
            match (bytes, reading_state.clone()) {
                ([x, FILE_CHAR, rest @ ..], DecodeState::ReadingVersionNum(2))
                    if *x != ESC_CHAR =>
                {
                    version.push(*x as char);
                    reading_state = DecodeState::ReadingFileName;
                    bytes = rest;
                }
                ([x @ 46, rest @ ..], DecodeState::ReadingVersionNum(y)) => {
                    reading_state = DecodeState::ReadingVersionNum(y + 1);
                    version.push(*x as char);
                    bytes = rest;
                }
                ([x, rest @ ..], DecodeState::ReadingVersionNum(_)) => {
                    version.push(*x as char);
                    bytes = rest;
                }
                ([BOWL_CHAR, VERSION_CHAR, rest @ ..], DecodeState::Reading) => {
                    reading_state = DecodeState::ReadingVersionNum(0);
                    bytes = rest;
                }
                ([x, FILE_CHAR, rest @ ..], DecodeState::Reading) if *x != ESC_CHAR => {
                    reading_state = DecodeState::ReadingFileName;
                    bytes = rest;
                }
                ([x, CONTENT_CHAR, rest @ ..], DecodeState::ReadingFileName) if *x != ESC_CHAR => {
                    file_path.push(*x as char);
                    reading_state = DecodeState::ReadingFileContent;
                    bytes = rest;
                }
                ([x, rest @ ..], DecodeState::ReadingFileName) => {
                    file_path.push(*x as char);
                    bytes = rest;
                }
                ([x, FILE_CHAR, rest @ ..], DecodeState::ReadingFileContent) if *x != ESC_CHAR => {
                    file_content.push(*x);
                    files.push(FileContent {
                        file_path: file_path.clone(),
                        content: unescape_content(file_content.clone()),
                    });
                    file_path = String::new();
                    file_content = Vec::new();
                    bytes = rest;
                    reading_state = DecodeState::ReadingFileName;
                }
                ([x, rest @ ..], DecodeState::ReadingFileContent) => {
                    file_content.push(*x);
                    bytes = rest;
                }
                ([], DecodeState::ReadingFileContent) => {
                    files.push(FileContent {
                        file_path: file_path.clone(),
                        content: unescape_content(file_content.clone()),
                    });
                    break;
                }
                _ => {
                    return Err("Invalid bowl file provided");
                }
            }
        }

        // TODO: validate that the version is actually in form X.X.X

        Ok(Self { version, files })
    }

    /// Parse a BowlFile from string
    pub fn from_string(_raw: String) -> Self {
        todo!()
    }

    /// Encode the files provided in the bowl format.
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.push(BOWL_CHAR);
        result.push(VERSION_CHAR);
        for b in CURRENT_VERSION.as_bytes() {
            result.push(*b);
        }
        for f in self.files.clone() {
            result.push(FILE_CHAR);
            for c in f.file_path.as_bytes() {
                result.push(*c);
            }
            result.push(CONTENT_CHAR);
            result.append(&mut escape_content(f.content));
        }

        result
    }
}

#[derive(Debug, Clone)]
pub struct FileContent {
    /// File path relative to the caller
    pub file_path: String,
    /// The inescaped content of the file
    pub content: Vec<u8>,
}

impl FileContent {
    pub fn from_path(path: PathBuf) -> Result<Self, String> {
        let file_path = path.clone().to_str().unwrap().to_owned();
        let content = fs::read(path).map_err(|e| e.to_string())?;
        Ok(Self { file_path, content })
    }
}

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
    fn test_encode_decode_content() {
        let bf = BowlFile {
            version: CURRENT_VERSION.to_owned(),
            files: vec![
                FileContent {
                    file_path: "README.md".into(),
                    content: "# My epic readme\nGamer".into(),
                },
                FileContent {
                    file_path: "src/main.rs".into(),
                    content: "pub fn main() {println!(\"Hello, world\")}".into(),
                },
            ],
        };

        let expected = vec![
            BOWL_CHAR,
            VERSION_CHAR,
            48,
            46,
            48,
            46,
            49,
            FILE_CHAR,
            82,
            69,
            65,
            68,
            77,
            69,
            46,
            109,
            100,
            CONTENT_CHAR,
            35,
            32,
            77,
            121,
            32,
            101,
            112,
            105,
            99,
            32,
            114,
            101,
            97,
            100,
            109,
            101,
            10,
            71,
            97,
            109,
            101,
            114,
            FILE_CHAR,
            115,
            114,
            99,
            47,
            109,
            97,
            105,
            110,
            46,
            114,
            115,
            CONTENT_CHAR,
            112,
            117,
            98,
            32,
            102,
            110,
            32,
            109,
            97,
            105,
            110,
            40,
            41,
            32,
            123,
            112,
            114,
            105,
            110,
            116,
            108,
            110,
            33,
            40,
            34,
            72,
            101,
            108,
            108,
            111,
            44,
            32,
            119,
            111,
            114,
            108,
            100,
            34,
            41,
            125,
        ];

        assert_eq!(bf.encode(), expected);
        let nbf = BowlFile::decode(expected).unwrap();

        assert_eq!(bf.files.len(), nbf.files.len());
        for i in 0..bf.files.len() {
            assert_eq!(bf.files[i].file_path, nbf.files[i].file_path);
            assert_eq!(bf.files[i].content, nbf.files[i].content);
        }
    }

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
