#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_pattern_chunk_matching() {
        let pattern = Pattern::new("00 ?? 00 ??").unwrap();
        let matches = pattern.match_chunk(vec![0, 42, 0, 13]);
        assert_eq!(matches, vec![42, 13])
    }

    #[test]
    fn test_pattern_whole_matching() {
        let pattern = Pattern::new("DEADBEEF ??").unwrap();
        let mut data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 23];
        for i in 0..100 {
            data.insert(0, i);
        }
        for i in 0..100 {
            data.push(i);
        }
        let matches = pattern.find_matches(data);
        assert_eq!(matches, vec![23]);
    }
}
