
pub struct BetterLinearSearch;

impl BetterLinearSearch {

    pub fn search<T>(data: &[T], target: &T) -> Option<usize> where T: PartialEq, {
        for (i, el) in data.iter().enumerate() {
            if el == target {
                return Some(i);
            }
        }
        return None;
    }
}

