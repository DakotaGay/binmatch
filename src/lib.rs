#[cfg(test)]
mod tests;

use color_eyre::eyre::{eyre, Result};

pub struct Pattern {
    data: Vec<PatternElement>,
    len: usize,
}

const ALLOWED_ALPHABET: [char; 17] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    '?', // ? is used to indicate a placeholder
];

#[derive(Clone)]
enum PatternElement {
    Literal(u8),
    Placeholder,
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Pattern> {
        let string = pattern.replace(" ", "").to_uppercase();
        for char in string.chars() {
            if !ALLOWED_ALPHABET.contains(&char) {
                return Err(eyre!(
                    "Invalid Character passed to binmatch::pattern::new [{}]",
                    char
                ));
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

        Ok(Self {
            data: data,
            len: string.len(),
        })
    }

    pub fn find_match(&self, haystack: Vec<u8>) -> Result<Vec<u8>> {
        let mut matches = vec![];
        for sub in haystack.windows(self.len) {
            matches.extend(self.match_chunk(sub.to_vec())?);
        }
        Ok(matches)
    }

    pub fn match_chunk(&self, chunk: Vec<u8>) -> Result<Vec<u8>> {
        let mut matches = vec![];
        for (actual, expected) in chunk.iter().zip(self.data.clone()) {
            match expected {
                PatternElement::Literal(expected) => {
                    if expected != *actual {
                        return Ok(vec![]); // Discard all matches
                    }
                }
                PatternElement::Placeholder => matches.push(*actual),
            }
        }
        Ok(matches)
    }
}
