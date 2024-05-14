use std::cmp::Ordering;

pub struct BinarySearch;

impl BinarySearch {

    pub fn search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
        // Edge cases
        if data.len() == 0 { return None; }
        if data[0] > *target || data[data.len()-1] < *target { return None; }

        Self::binary_search(data, target, 0, data.len()-1)
    }

    fn binary_search<T: Ord>(data: &[T], target: &T, first: usize, last: usize) -> Option<usize> {

        if first > last { // Item not found
            return None;
        }

        // Divide the vector in half
        let mid: usize = (first + last) / 2;

        match data[mid].cmp(target) { // Determine which half to search
            Ordering::Equal => Some(mid), // Base case
            Ordering::Less => Self::binary_search(data, target, mid + 1, last),
            Ordering::Greater => Self::binary_search(data, target, first, mid - 1),
        }
    }
}

