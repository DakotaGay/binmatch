//! A simple binary pattern matching library
//!
//! Basic Usage looks like this:
//! ```
//! # use binmatch::Pattern;
//! let pattern = Pattern::new("00 00 ??").unwrap();
//! let data = vec![0x12, 0x13, 0x14, 0x00, 0x00, 0x42, 0x15];
//! let matches = pattern.find_matches(data);
//! assert_eq!(matches, vec![0x42]);
//! ```
//!
//! All needed functions can be found in [Pattern]

use std::fmt;

#[cfg(test)]
mod tests;

const ALLOWED_ALPHABET: [char; 17] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    '?', // ? is used to indicate a placeholder
];

#[derive(Debug)]
pub enum BinmatchError {
    PatternParseError(char),
}

impl fmt::Display for BinmatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PatternParseError(c) => {
                write!(
                    f,
                    "Invalid Character passed to binmatch::pattern::new [{}]",
                    c
                )
            }
        }
    }
}
impl std::error::Error for BinmatchError {}

#[derive(Clone, Debug)]
pub struct Pattern {
    data: Vec<PatternElement>,
    len: usize,
}

#[derive(Clone, Copy, Debug)]
enum PatternElement {
    Literal(u8),
    Placeholder,
}

impl Pattern {
    /// Create a new `Pattern`  
    /// Only use characters in the hexadecimal numbering system and question marks  
    /// Use ?? as a placeholder  
    /// Spaces are ignored  
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 00 ??").unwrap();
    /// ```
    pub fn new(pattern: &str) -> Result<Pattern, Box<dyn std::error::Error>> {
        let string = pattern.replace(' ', "").to_uppercase();
        for char in string.chars() {
            if !ALLOWED_ALPHABET.contains(&char) {
                return Err(Box::new(BinmatchError::PatternParseError(char)));
            }
        }

        let mut data: Vec<PatternElement> = vec![];
        for hex in string.chars().collect::<Vec<char>>().chunks(2) {
            let hex = String::from_utf8(hex.to_vec().iter().map(|&c| c as u8).collect())?;
            if hex == "??" {
                data.push(PatternElement::Placeholder);
            } else {
                data.push(PatternElement::Literal(u8::from_str_radix(&hex, 16)?))
            }
        }
        let len = data.len();

        Ok(Self { data, len })
    }

    /// Finds all matches in the `haystack`
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 00 ??").unwrap();
    /// let data = vec![0xFF, 0x12, 0x34, 0x00, 0x00, 0x42, 0x56, 0x78];
    /// let matches = pattern.find_matches(data);
    /// assert_eq!(matches, vec![0x42]);
    /// ```
    pub fn find_matches(&self, haystack: Vec<u8>) -> Vec<u8> {
        let mut matches = vec![];
        for sub in haystack.windows(self.len) {
            matches.extend(self.match_chunk(sub.to_vec()));
        }
        matches
    }

    /// Finds a match in a chunk
    /// Called by [Pattern::find_matches]
    ///
    /// Pattern.len **MUST** be the same size as chunk.len
    ///
    /// # Examples:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 00 ??").unwrap();
    /// let matches = pattern.match_chunk(vec![0x00, 0x00, 0x42]);
    /// assert_eq!(matches, vec![0x42]);
    /// ```
    ///
    /// ```should_panic
    /// # use binmatch::Pattern;
    /// // This will panic
    /// let pattern = Pattern::new("00 00 ??").unwrap();
    /// let matches = pattern.match_chunk(vec![0x00, 0x00, 0x42, 0x00]);
    /// unreachable!();
    /// ```
    pub fn match_chunk(&self, chunk: Vec<u8>) -> Vec<u8> {
        assert_eq!(self.len, chunk.len());
        let mut matches = vec![];
        for (actual, expected) in chunk.iter().zip(self.data.clone()) {
            match expected {
                PatternElement::Literal(expected) => {
                    if expected != *actual {
                        return vec![]; // Discard all matches
                    }
                }
                PatternElement::Placeholder => matches.push(*actual),
            }
        }
        matches
    }
}
