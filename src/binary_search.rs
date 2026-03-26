use std::cmp::Ordering::*;

pub struct BinarySearch;

impl BinarySearch {

    pub fn search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
        let mid: usize = data.len() / 2;
        Some(match data.get(mid)?.cmp(target) { // `?` causes None return when data is empty
            Greater => Self::search(&data[..mid], target)?, // Left recur
            Equal => mid, // Base case
            Less => mid + 1 + Self::search(&data[mid+1..], target)?, // Right recur
        })
    }
}

