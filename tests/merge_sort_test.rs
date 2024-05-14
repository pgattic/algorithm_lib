
mod merge_sort_test {
    use algorithm_lib::MergeSort;
    
    #[test]
    fn test_sort_even_count() {
        let sorted_array = vec![1, 2, 3, 4, 5, 6 ];
        let mut unsorted_array = vec![3, 5, 2, 6, 1, 4 ];
        MergeSort::merge_sort(&mut unsorted_array);
        assert_eq!(sorted_array, unsorted_array);
    }
    
    #[test]
    fn test_sort_odd_count() {
        let sorted_array = vec![1, 2, 3, 4, 5, 6, 7 ];
        let mut unsorted_array = vec![3, 5, 7, 2, 6, 1, 4 ];
        MergeSort::merge_sort(&mut unsorted_array);
        assert_eq!(sorted_array, unsorted_array);
    }
    
    #[test]
    fn test_already_sorted() {
        let mut sorted_array = vec![1, 2, 3, 4, 5, 6 ];
        let data = vec![1, 2, 3, 4, 5, 6];
        MergeSort::merge_sort(&mut sorted_array);
        assert_eq!(sorted_array, data);
    }
    
    #[test]
    fn test_empty() {
        let mut sorted_array: Vec<i32> = vec![];
        let data = vec![];
        MergeSort::merge_sort(&mut sorted_array);
        assert_eq!(sorted_array, data);
    }
}

