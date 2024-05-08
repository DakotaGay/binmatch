#[cfg(feature = "std")]
mod std_tests {
    use crate::Pattern;

    #[test]
    fn test_pattern_chunk_matching() {
        let pattern = Pattern::new("00 ?? 00 ??").unwrap();
        let (matches, _) = pattern.match_chunk(vec![0, 42, 0, 13]);
        assert_eq!(matches, vec![(42, 1), (13, 3)])
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
        let matches = pattern.find_matches_with_index(data);
        assert_eq!(matches, vec![(23, 100 + 4)]);
    }

    #[test]
    fn test_convenience_method() {
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

    #[test]
    fn ignore_some_bytes() {
        let pattern = Pattern::new("00 __ 12 __ ??").unwrap();
        let data: Vec<u8> = vec![0x00, 0xFF, 0x12, 0xAA, 23];
        let matches = pattern.find_matches(data);
        assert_eq!(matches, vec![23]);
    }
}

#[cfg(not(feature = "std"))]
mod no_std_tests {
    use crate::Pattern;
    use alloc::vec::Vec;

    #[test]
    fn test_pattern_chunk_matching() {
        let pattern = Pattern::new_unchecked("00 ?? 00 ??");
        let (matches, _) = pattern.match_chunk([0, 42, 0, 13].to_vec());
        assert_eq!(matches, [(42, 1), (13, 3)].to_vec())
    }

    #[test]
    fn test_pattern_whole_matching() {
        let pattern = Pattern::new_unchecked("DEADBEEF ??");
        let mut data: Vec<u8> = [0xDE, 0xAD, 0xBE, 0xEF, 23].to_vec();
        for i in 0..100 {
            data.insert(0, i);
        }
        for i in 0..100 {
            data.push(i);
        }
        let matches = pattern.find_matches_with_index(data);
        assert_eq!(matches, [(23, 100 + 4)].to_vec());
    }

    #[test]
    fn test_convenience_method() {
        let pattern = Pattern::new_unchecked("DEADBEEF ??");
        let mut data: Vec<u8> = [0xDE, 0xAD, 0xBE, 0xEF, 23].to_vec();
        for i in 0..100 {
            data.insert(0, i);
        }
        for i in 0..100 {
            data.push(i);
        }
        let matches = pattern.find_matches(data);
        assert_eq!(matches, [23].to_vec());
    }

    #[test]
    fn ignore_some_bytes() {
        let pattern = Pattern::new_unchecked("00 __ 12 __ ??");
        let data: Vec<u8> = [0x00, 0xFF, 0x12, 0xAA, 23].to_vec();
        let matches = pattern.find_matches(data);
        assert_eq!(matches, [23].to_vec());
    }
}
