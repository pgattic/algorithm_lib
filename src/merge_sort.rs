
pub struct MergeSort;

impl MergeSort {
    pub fn merge_sort<T: Ord + Copy>(data: &mut [T]) {
        let len = data.len();
        if len <= 1 { // Base case: List of 1 item is trivially sorted
            return;
        }

        let mid = len / 2;
        let left = &mut data[..mid].to_vec(); 
        let right = &mut data[mid..].to_vec();

        Self::merge_sort(left); // Recur on left side
        Self::merge_sort(right); // Recur on right side

        Self::merge(left, right, data); // Merge resultant vecs, assuming sorted data
    }

    fn merge<T: Ord + Copy>(left: &mut [T], right: &mut [T], data: &mut [T]) {
        let mut l_i = 0;
        let mut r_i = 0;
        let mut d_i = 0;

        while l_i < left.len() && r_i < right.len() { // While both sides have data
            if left[l_i] <= right[r_i] { // pick smaller side to grab data from
                data[d_i] = left[l_i];
                l_i += 1;
            } else {
                data[d_i] = right[r_i];
                r_i += 1;
            }
            d_i += 1;
        }

        while l_i < left.len() { // If the right side empties first
            data[d_i] = left[l_i];
            l_i += 1;
            d_i += 1;
        }

        while r_i < right.len() { // If the left side empties first
            data[d_i] = right[r_i];
            r_i += 1;
            d_i += 1;
        }
    }
}

