//! A simple binary pattern matching library
//!
//!
//! Basic Usage looks like this:
//! ```
//! # use binmatch::Pattern;
//! let pattern = Pattern::new("00 __ 00 ??").unwrap();
//! let data = vec![0x12, 0x13, 0x00, 0x14, 0x00, 0x42, 0x15];
//! let matches = pattern.find_matches(data); // Or Pattern::find_matches_with_index if you need the index
//! assert_eq!(matches, vec![0x42]);
//! ```
//!
//! All needed functions can be found in [Pattern]
//!
//! # Usage with `#![no_std]`
//! First off, disable the default feature `std`  
//! `cargo add binmatch --no-default-features`  
//! The normal [Pattern::new] is no longer accesible, because it needs `std` to function  
//! Every time you wish to create a new [Pattern] you now have to use [Pattern::new_unchecked]  
//!

#[cfg(not(feature = "std"))]
include!("no_std_include.rs");
#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(test)]
mod tests;

pub const ALLOWED_ALPHABET: [char; 18] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    '?', // ? is used to indicate a placeholder
    '_', // _ is used to indicate a character to ignore
];

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum BinmatchError {
    #[error("Invalid Character passed to binmatch::pattern::new [{0}]")]
    PatternParseError(char),
    #[error("Patterns should always be an even number of characters long")]
    PatternLengthError,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Pattern {
    data: Vec<PatternElement>,
    len: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum PatternElement {
    Literal(u8),
    Placeholder,
    Ignore,
}

impl Pattern {
    /// <div class="warning"> Only available using the <code>std</code> feature </div>
    ///
    /// Create a new `Pattern`  
    ///
    ///
    /// # Returns an Error when:
    ///
    /// - The input `&str` contains Characters not contained in `ALLOWED_ALPHABET`
    /// - The inputs length is not divisible by 2
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 __ 00 ??").unwrap();
    /// ```
    #[cfg(feature = "std")]
    pub fn new(pattern: &str) -> Result<Pattern, Box<dyn std::error::Error>> {
        let string = pattern.replace(' ', "").to_uppercase();
        if string.len() % 2 != 0 {
            return Err(Box::new(BinmatchError::PatternLengthError));
        }
        for char in string.chars() {
            if !ALLOWED_ALPHABET.contains(&char) {
                return Err(Box::new(BinmatchError::PatternParseError(char)));
            }
        }

        let mut data: Vec<PatternElement> = Vec::new();
        for hex in string.chars().collect::<Vec<char>>().chunks(2) {
            let hex = String::from_utf8(hex.to_vec().iter().map(|&c| c as u8).collect())?;
            match hex.as_str() {
                "??" => data.push(PatternElement::Placeholder),
                "__" => data.push(PatternElement::Ignore),
                v => data.push(PatternElement::Literal(u8::from_str_radix(v, 16)?)),
            }
        }
        let len = data.len();

        Ok(Self { data, len })
    }

    /// Create a new `Pattern`  
    ///
    /// # Panics when:
    /// - The input `&str` contains Characters not contained in `ALLOWED_ALPHABET`
    /// - The inputs length is not divisible by 2
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new_unchecked("00 __ 00 ??");
    /// ```
    pub fn new_unchecked(pattern: &str) -> Pattern {
        let string = pattern.replace(' ', "").to_uppercase();
        assert!(string.len() % 2 == 0);
        for char in string.chars() {
            assert!(ALLOWED_ALPHABET.contains(&char));
        }

        let mut data: Vec<PatternElement> = Vec::new();
        for hex in string.chars().collect::<Vec<char>>().chunks(2) {
            let hex = String::from_utf8(hex.to_vec().iter().map(|&c| c as u8).collect())
                .expect("Could not parse a chunk to a String");
            match hex.as_str() {
                "??" => data.push(PatternElement::Placeholder),
                "__" => data.push(PatternElement::Ignore),
                v => data.push(PatternElement::Literal(
                    u8::from_str_radix(v, 16).expect("Could not parse the string to a u8"),
                )), // It shouldn't be possible to panic from this line
            }
        }
        let len = data.len();

        Self { data, len }
    }

    /// Finds all matches in the `haystack`
    ///
    /// Use [Pattern::find_matches] if you don't need the index
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("34 __ 00 ??").unwrap();
    /// let data = vec![0xFF, 0x12, 0x34, 0x12, 0x00, 0x42, 0x56, 0x78];
    /// let matches = pattern.find_matches_with_index(data);
    /// assert_eq!(matches, vec![(0x42, 5)]);
    /// ```
    pub fn find_matches_with_index(&self, haystack: Vec<u8>) -> Vec<(u8, usize)> {
        let mut matches = Vec::new();
        for (i, sub) in haystack.windows(self.len).enumerate() {
            matches.extend(
                self.match_chunk(sub.to_vec())
                    .0
                    .iter()
                    .map(|m| (m.0, m.1 + i))
                    .collect::<Vec<(u8, usize)>>(),
            );
        }
        matches
    }

    /// Convenience Method for cases where the index is not needed
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 __ 00 ??").unwrap();
    /// let data = vec![0xFF, 0x12, 0x34, 0x00, 0x32, 0x00, 0x42, 0x56, 0x78];
    /// let matches = pattern.find_matches(data);
    /// assert_eq!(matches, vec![0x42]);
    /// ```
    pub fn find_matches(&self, haystack: Vec<u8>) -> Vec<u8> {
        let matches = self.find_matches_with_index(haystack);
        matches.iter().map(|(matched, _)| *matched).collect()
    }

    /// Convenience Method for cases where the values are irrelevant and you only need to know if the Pattern matches the data
    ///
    /// # Example:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 __ 00 __").unwrap();
    /// let data = vec![0xFF, 0x12, 0x34, 0x00, 0x32, 0x00, 0x42, 0x56, 0x78];
    /// assert_eq!(pattern.has_match(data), true);
    /// ```
    pub fn has_match(&self, haystack: Vec<u8>) -> bool {
        for sub in haystack.windows(self.len) {
            if self.match_chunk(sub.to_vec()).1 {
                return true;
            }
        }
        false
    }

    /// Finds a match in a chunk  
    /// Called by [Pattern::find_matches]  
    /// You normally don't need to use this
    ///
    /// `Pattern.len()` **MUST** be the same size as `chunk.len()`
    ///
    /// Returns a Vec of Tuples of the matched Value and the Index
    ///
    /// # Examples:
    /// ```
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 __ 00 ??").unwrap();
    /// let (matches, _) = pattern.match_chunk(vec![0x00, 0x32 ,0x00, 0x42]);
    /// assert_eq!(matches, vec![(0x42, 3)]);
    /// ```
    ///
    /// ```should_panic
    /// # use binmatch::Pattern;
    /// let pattern = Pattern::new("00 __ 00 ??").unwrap();
    /// let (matches, _) = pattern.match_chunk(vec![0x00, 0x32, 0x42, 0x00, 0x00]);
    /// unreachable!();
    /// ```
    pub fn match_chunk(&self, chunk: Vec<u8>) -> (Vec<(u8, usize)>, bool) {
        assert_eq!(self.len, chunk.len());
        let mut matches = Vec::new();
        for (index, (actual, expected)) in chunk.iter().zip(self.data.clone()).enumerate() {
            match expected {
                PatternElement::Literal(expected) => {
                    if expected != *actual {
                        return (Vec::new(), false); // Discard all matches
                    }
                }
                PatternElement::Placeholder => matches.push((*actual, index)),
                PatternElement::Ignore => (),
            }
        }
        (matches, true)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
