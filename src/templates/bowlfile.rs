//! This file contains information about the encoding of Bowl files
//!
//! The encoding is as follows (capitalized means constant mapped to u8):
//!
//! BOWLFILE: BOWL + VERSION + F1 + F2 + F3 + ...
//! FILE IN BOWLFILE: BOWL + FILE + <filename> + CONTENT + <escaped file contents>.
//!
//! *NOTE: This module will likely be broken into multiple later on*
//! For example, I want to create separate files for versions of the BowlFile

use super::escape::{escape_content, unescape_content};
use super::files::FileContent;
use super::symbols::{BOWL_CHAR, CONTENT_CHAR, CURRENT_VERSION, ESC_CHAR, FILE_CHAR, VERSION_CHAR};

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
                        file_path,
                        content: unescape_content(file_content),
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
                        file_path,
                        content: unescape_content(file_content),
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_encode_decode_content() {
        // ensures some level of bijectivity between these two functions
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
}
