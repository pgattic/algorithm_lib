
pub mod better_linear_search;
pub mod binary_search;
pub mod merge_sort;
pub mod quick_sort;

// Re-export BetterLinearSearch so it's available at the top level.
pub use better_linear_search::BetterLinearSearch;
pub use binary_search::BinarySearch;
pub use merge_sort::MergeSort;
pub use quick_sort::QuickSort;

