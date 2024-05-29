//! This file contains information about the encoding of Bowl files
//!
//! The encoding is as follows (capitalized means constant mapped to u8):
//!
//! BOWLFILE: BOWL + VERSION + F1 + F2 + F3 + ...
//! FILE IN BOWLFILE: BOWL + FILE + <filename> + CONTENT + <escaped file contents>.
//!
//! *NOTE: This module will likely be broken into multiple later on*

use std::{fs, path::PathBuf};

const ESC_CHAR: u8 = 0xFF;
const BOWL_CHAR: u8 = 0x9A;
const FILE_CHAR: u8 = 0x9C;
const CONTENT_CHAR: u8 = 0x9E;
const VERSION_CHAR: u8 = 0xA0;

const CURRENT_VERSION: &'static str = "0";

/// Represents the parsed version of a bowl template
pub struct BowlFile {
    pub version: String,
    pub files: Vec<FileContent>,
}

impl BowlFile {
    pub fn new(files: Vec<FileContent>) -> Self {
        Self {
            version: CURRENT_VERSION.to_owned(),
            files,
        }
    }

    /// Parse a BowlFile from string
    pub fn from_string(_raw: String) -> BowlFile {
        todo!()
    }
    /// Encode the files provided in the bowl format.
    pub fn encode(self) -> Vec<u8> {
        let mut result = Vec::new();

        result.push(BOWL_CHAR);
        result.push(VERSION_CHAR);
        for b in CURRENT_VERSION.as_bytes() {
            result.push(*b);
        }
        for f in self.files {
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

#[derive(Debug)]
pub struct FileContent {
    /// File path relative to the caller
    pub file_path: String,
    /// The inescaped content of the file
    pub content: String,
}

impl FileContent {
    pub fn from_path(path: PathBuf) -> Result<Self, String> {
        let file_path = path.clone().to_str().unwrap().to_owned();
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Ok(Self { file_path, content })
    }
}

/// Replace bowl char sequences with escaped ones to avoid
/// problems in decoding
pub fn escape_content(content: String) -> Vec<u8> {
    let mut result = Vec::new();
    let bytes = content.as_bytes();

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
pub fn unescape_content(content: Vec<u8>) -> String {
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

    String::from_utf8(result).expect("Found invalid UTF-8")
}

///
pub fn decode_content(_raw: String) -> BowlFile {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_content() {
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
    }
}
