
mod binary_search_test {
    use algorithm_lib::BinarySearch;

    #[test]
    fn test_match_first() {
        let data = vec![ 1, 3, 6, 7, 11, 13, 15 ];
        let target = 1;
        assert_eq!(BinarySearch::search(&data, &target), Some(0));
    }

    #[test]
    fn test_no_match_last() {
        let data = vec![ 1, 3, 6, 7, 11, 13, 15 ];
        let target = 15;
        assert_eq!(BinarySearch::search(&data, &target), Some(6));
    }

    #[test]
    fn test_match_middle() {
        let data = vec![ 1, 3, 6, 7, 11, 13, 15 ];
        let target = 7;
        assert_eq!(BinarySearch::search(&data, &target), Some(3));
    }

    #[test]
    fn test_no_match_too_big() {
        let data = vec![ 1, 3, 6, 7, 11, 13, 15 ];
        let target = 20;
        assert_eq!(BinarySearch::search(&data, &target), None);
    }

    #[test]
    fn test_no_match_too_small() {
        let data = vec![ 1, 3, 6, 7, 11, 13, 15 ];
        let target = 0;
        assert_eq!(BinarySearch::search(&data, &target), None);
    }

    #[test]
    fn test_no_match_middle() {
        let data = vec![ 1, 3, 6, 7, 11, 13, 15 ];
        let target = 4;
        assert_eq!(BinarySearch::search(&data, &target), None);
    }

    #[test]
    fn test_empty() {
        let data: Vec<i32> = Vec::new();
        let target = 7;
        assert_eq!(BinarySearch::search(&data, &target), None);
    }
}

