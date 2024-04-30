
mod linear_search_test {
    use algorithm_lib::BetterLinearSearch;

    #[test]
    fn test_match_number() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let target = 4;
        assert_eq!(BetterLinearSearch::search(&data, &target), Some(3));
    }

    #[test]
    fn test_no_match_number() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let target = 8;
        assert_eq!(BetterLinearSearch::search(&data, &target), None);
    }

    #[test]
    fn test_match_string() {
        let data = vec!["cat", "dog", "pig", "chicken", "duck"];
        let target = "pig";
        assert_eq!(BetterLinearSearch::search(&data, &target), Some(2));
    }

    #[test]
    fn test_no_match_string() {
        let data = vec!["cat", "dog", "pig", "chicken", "duck"];
        let target = "eagle";
        assert_eq!(BetterLinearSearch::search(&data, &target), None);
    }

    #[test]
    fn test_empty() {
        let data: Vec<i32> = Vec::new();
        let target = 6;
        assert_eq!(BetterLinearSearch::search(&data, &target), None);
    }

    #[test]
    fn test_search_duplicates() {
        let data = vec![1, 2, 3, 3, 4, 5];
        let target = 3;
        assert_eq!(BetterLinearSearch::search(&data, &target), Some(2));
    }
}

