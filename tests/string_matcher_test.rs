
mod string_matcher_test {

    use algorithm_lib::StringMatcher;

    #[test]
    fn test_build_table_1() {
        let results = StringMatcher::build_table("AAC".to_string(), "ACGT".to_string());
        assert_eq!(*results[0].get(&b'A').unwrap(), 1);
        assert_eq!(*results[0].get(&b'C').unwrap(), 0);
        assert_eq!(*results[0].get(&b'G').unwrap(), 0);
        assert_eq!(*results[0].get(&b'T').unwrap(), 0);
        assert_eq!(*results[1].get(&b'A').unwrap(), 2);
        assert_eq!(*results[1].get(&b'C').unwrap(), 0);
        assert_eq!(*results[1].get(&b'G').unwrap(), 0);
        assert_eq!(*results[1].get(&b'T').unwrap(), 0);
        assert_eq!(*results[2].get(&b'A').unwrap(), 2);
        assert_eq!(*results[2].get(&b'C').unwrap(), 3);
        assert_eq!(*results[2].get(&b'G').unwrap(), 0);
        assert_eq!(*results[2].get(&b'T').unwrap(), 0);
        assert_eq!(*results[3].get(&b'A').unwrap(), 1);
        assert_eq!(*results[3].get(&b'C').unwrap(), 0);
        assert_eq!(*results[3].get(&b'G').unwrap(), 0);
        assert_eq!(*results[3].get(&b'T').unwrap(), 0);
        assert_eq!(results.len(), 4);
    }
    
    #[test]
    fn test_build_table_2() {
        let results = StringMatcher::build_table("CBCBA".to_string(), "ABC".to_string());
        assert_eq!(*results[0].get(&b'A').unwrap(), 0);
        assert_eq!(*results[0].get(&b'B').unwrap(), 0);
        assert_eq!(*results[0].get(&b'C').unwrap(), 1);
        assert_eq!(*results[1].get(&b'A').unwrap(), 0);
        assert_eq!(*results[1].get(&b'B').unwrap(), 2);
        assert_eq!(*results[1].get(&b'C').unwrap(), 1);
        assert_eq!(*results[2].get(&b'A').unwrap(), 0);
        assert_eq!(*results[2].get(&b'B').unwrap(), 0);
        assert_eq!(*results[2].get(&b'C').unwrap(), 3);
        assert_eq!(*results[3].get(&b'A').unwrap(), 0);
        assert_eq!(*results[3].get(&b'B').unwrap(), 4);
        assert_eq!(*results[3].get(&b'C').unwrap(), 1);
        assert_eq!(*results[4].get(&b'A').unwrap(), 5);
        assert_eq!(*results[4].get(&b'B').unwrap(), 0);
        assert_eq!(*results[4].get(&b'C').unwrap(), 3);
        assert_eq!(*results[5].get(&b'A').unwrap(), 0);
        assert_eq!(*results[5].get(&b'B').unwrap(), 0);
        assert_eq!(*results[5].get(&b'C').unwrap(), 1);
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn test_match_1() {
        let results = StringMatcher::find_match("GTAACAGTAAACG", "AAC".to_string(), "ACGT".to_string());
        assert_eq!(results, vec![4, 11]);
    }

    #[test]
    fn test_match_2() {
        let results = StringMatcher::find_match("GTAACAGTAAACG", "AA".to_string(), "ACGT".to_string());
        assert_eq!(results, vec![3, 9, 10]);
    }

    #[test]
    fn test_match_3() {
        let results = StringMatcher::find_match("ABCBCABCBCBC", "CBC".to_string(), "ABC".to_string());
        assert_eq!(results, vec![4, 9, 11]);
    }

    #[test]
    fn test_match_4() { // No matches
        let results = StringMatcher::find_match("GTAACAGTAAACG", "AACT".to_string(), "ACGT".to_string());
        assert_eq!(results, vec![]);
    }

}
