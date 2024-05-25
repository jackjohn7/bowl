//! This file contains information about the encoding of Bowl files
//!
//! The encoding is as follows (capitalized means constant mapped to u8):
//!
//! BOWLFILE: BOWL + VERSION + F1 + F2 + F3 + ...
//! FILE IN BOWLFILE: BOWL + FILE + <filename> + CONTENT + <escaped file contents>.
//!
//! *NOTE: This module will likely be broken into multiple later on*

use std::path::PathBuf;

const ESC_CHAR: u8 = 0xFF;
const BOWL_CHAR: u8 = 0x9A;
const FILE_CHAR: u8 = 0x9C;
const CONTENT_CHAR: u8 = 0x9E;
const VERSION_CHAR: u8 = 0xA0;

const CURRENT_VERSION: &'static str = "0";

/// Represents the parsed version of a bowl template
pub struct BowlFile {
    pub version: u16,
    pub files: Vec<FileContent>,
}

impl BowlFile {
    /// Parse a BowlFile from string
    pub fn from_string(_raw: String) -> BowlFile {
        todo!()
    }
}

pub struct FileContent {
    /// File path relative to the caller
    pub file_path: String,
    /// The inescaped content of the file
    pub content: String,
}

impl FileContent {
    pub fn from_path(_path: PathBuf) {
        todo!()
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

/// Encode the files provided in the bowl format.
pub fn encode_content(file: BowlFile) -> Vec<u8> {
    let mut result = Vec::new();

    result.push(BOWL_CHAR);
    result.push(VERSION_CHAR);
    for b in CURRENT_VERSION.as_bytes() {
        result.push(*b);
    }
    for f in file.files {
        result.push(FILE_CHAR);
        for c in f.file_path.as_bytes() {
            result.push(*c);
        }
        result.push(CONTENT_CHAR);
        result.append(&mut escape_content(f.content));
    }

    result
}

///
pub fn decode_content(_raw: String) -> BowlFile {
    todo!()
}
