
mod string_matcher_test {

    use algorithm_lib::StringMatcher;

    #[test]
    fn test1_build_table() {
        let results = StringMatcher::build_table("AAC", "ACGT");
        assert_eq!(*results[0].get(&'A').unwrap(), 1);
        assert_eq!(*results[0].get(&'C').unwrap(), 0);
        assert_eq!(*results[0].get(&'G').unwrap(), 0);
        assert_eq!(*results[0].get(&'T').unwrap(), 0);
        assert_eq!(*results[1].get(&'A').unwrap(), 2);
        assert_eq!(*results[1].get(&'C').unwrap(), 0);
        assert_eq!(*results[1].get(&'G').unwrap(), 0);
        assert_eq!(*results[1].get(&'T').unwrap(), 0);
        assert_eq!(*results[2].get(&'A').unwrap(), 2);
        assert_eq!(*results[2].get(&'C').unwrap(), 3);
        assert_eq!(*results[2].get(&'G').unwrap(), 0);
        assert_eq!(*results[2].get(&'T').unwrap(), 0);
        assert_eq!(*results[3].get(&'A').unwrap(), 1);
        assert_eq!(*results[3].get(&'C').unwrap(), 0);
        assert_eq!(*results[3].get(&'G').unwrap(), 0);
        assert_eq!(*results[3].get(&'T').unwrap(), 0);
        assert_eq!(results.len(), 4);
    }
    
    #[test]
    fn test2_build_table() {
        let results = StringMatcher::build_table("CBCBA", "ABC");
        assert_eq!(*results[0].get(&'A').unwrap(), 0);
        assert_eq!(*results[0].get(&'B').unwrap(), 0);
        assert_eq!(*results[0].get(&'C').unwrap(), 1);
        assert_eq!(*results[1].get(&'A').unwrap(), 0);
        assert_eq!(*results[1].get(&'B').unwrap(), 2);
        assert_eq!(*results[1].get(&'C').unwrap(), 1);
        assert_eq!(*results[2].get(&'A').unwrap(), 0);
        assert_eq!(*results[2].get(&'B').unwrap(), 0);
        assert_eq!(*results[2].get(&'C').unwrap(), 3);
        assert_eq!(*results[3].get(&'A').unwrap(), 0);
        assert_eq!(*results[3].get(&'B').unwrap(), 4);
        assert_eq!(*results[3].get(&'C').unwrap(), 1);
        assert_eq!(*results[4].get(&'A').unwrap(), 5);
        assert_eq!(*results[4].get(&'B').unwrap(), 0);
        assert_eq!(*results[4].get(&'C').unwrap(), 3);
        assert_eq!(*results[5].get(&'A').unwrap(), 0);
        assert_eq!(*results[5].get(&'B').unwrap(), 0);
        assert_eq!(*results[5].get(&'C').unwrap(), 1);
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn test3_match() {
        let results = StringMatcher::find_matches("GTAACAGTAAACG", "AAC", "ACGT");
        assert_eq!(results, vec![4, 11]);
    }

    #[test]
    fn test4_match() {
        let results = StringMatcher::find_matches("GTAACAGTAAACG", "AA", "ACGT");
        assert_eq!(results, vec![3, 9, 10]);
    }

    #[test]
    fn test5_match() {
        let results = StringMatcher::find_matches("ABCBCABCBCBC", "CBC", "ABC");
        assert_eq!(results, vec![4, 9, 11]);
    }

    #[test]
    fn test6_match() { // No matches
        let results = StringMatcher::find_matches("GTAACAGTAAACG", "AACT", "ACGT");
        assert_eq!(results, vec![]);
    }

}
