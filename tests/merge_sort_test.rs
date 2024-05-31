
mod merge_sort_test {
    use algorithm_lib::MergeSort;


//    #[test]
//    fn test_merge_equal_sized_sublists() {
//        let expected = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let left = vec![1, 4, 6, 7, 8];
//        let right = vec![2, 3, 5, 9, 10];
//        let mut result = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//        MergeSort::merge(&mut left, &mut right, &mut result);
//        assert_eq!(expected, result);
//    }
//
//    #[test]
//    fn test_merge_unequal_sized_sublists1() {
//        let expected = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let starting = vec![ 1, 4, 6, 7, 8, 9, 2, 3, 5, 10];
//        MergeSort.Merge(starting, 1, 5, 8);
//        Assert.That(starting.SequenceEqual(expected), Is.True);
//        Assert.Pass();
//    }
//    
//    #[test]
//    fn test_merge_unequal_sized_sublists2() {
//        let expected = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let starting = vec![ 1, 2, 4, 6, 3, 5, 7, 8, 9, 10];
//        MergeSort.Merge(starting, 1, 3, 8);
//        Assert.That(starting.SequenceEqual(expected), Is.True);
//        Assert.Pass();
//    }
//    
//    #[test]
//    fn test_merge_already_sorted() {
//        let expected = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let starting = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        MergeSort.Merge(starting, 1, 4, 8);
//        Assert.That(starting.SequenceEqual(expected), Is.True);
//        Assert.Pass();
//    }
//    
//    #[test]
//    fn test_merge_small1() {
//        let expected = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let starting = vec![ 1, 2, 3, 4, 6, 5, 7, 8, 9, 10];
//        MergeSort.Merge(starting, 4, 4, 6);
//        Assert.That(starting.SequenceEqual(expected), Is.True);
//        Assert.Pass();
//    }
//
//    #[test]
//    fn test_merge_small2() {
//        let expected = vec![ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//        let starting = vec![ 1, 2, 3, 4, 6, 7, 5, 8, 9, 10];
//        MergeSort.Merge(starting, 4, 5, 6);
//        Assert.That(starting.SequenceEqual(expected), Is.True);
//        Assert.Pass();
//    }

    
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

