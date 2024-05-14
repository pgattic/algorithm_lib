
pub struct QuickSort;

impl QuickSort {
    pub fn quick_sort<T: Ord>(data: &mut [T]) {
        if data.len() <= 1 {
            return // Base case: list of 1 is trivially sorted
        }
        let index = Self::partition(data); // Use the index returned as the split point

        Self::quick_sort(&mut data[..index]); // Mutable views into the vec.
        Self::quick_sort(&mut data[index+1..]); // Manually passing index limits isn't necessary.
    }

    fn partition<T: Ord>(data: &mut [T]) -> usize {
        let pivot = data.len()-1; // Using the last item as the pivot. Any index would work.
        let mut lmgp = 0; // Left-Most Greater than Pivot

        for i in 0..pivot {
            if data[i] <= data[pivot] {
                data.swap(i, lmgp);
                lmgp += 1;
            }
        }

        data.swap(lmgp, pivot); // The pivot value goes where lmgp was
        lmgp // Location of the value dividing the partitions
    }
}

