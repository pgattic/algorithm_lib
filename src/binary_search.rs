
pub struct BinarySearch;

impl BinarySearch {

    pub fn search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
        // Edge cases
        if data.len() == 0 { return None; }
        if data[0] > *target || data[data.len()-1] < *target { return None; }

        return Self::binary_search(&data, &target, 0, data.len()-1);
    }

    fn binary_search<T: Ord>(data: &[T], target: &T, first: usize, last: usize) -> Option<usize> {
        if first > last {
            return None;
        }

        // Divide the vector in half, test either side
        let mid: usize = (last-first)/2 + first;

        if data[mid] == *target {
            Some(mid)
        } else if data[mid] < *target {
            Self::binary_search(&data, &target, mid + 1, last)
        } else {
            Self::binary_search(&data, &target, first, mid - 1)
        }
    }
}

