use std::collections::HashMap;

pub struct PriorityQueue<T> {
    heap: Vec<T>,
    lookup: HashMap<T, usize>,
}

struct PQNode<T> {
    value: T,
    priority: usize,
}

//impl PriorityQueue<T> {
//    pub fn parent(index: usize) -> usize {
//        (index - 1) / 2
//    }
//    fn is_leaf(&self, index: usize) -> bool {
//        index >= self.heap.len() / 2
//    }
//    fn left(index: usize) -> usize {
//        2*index + 1
//    }
//    fn right(index: usize) -> usize {
//        2*index + 2
//    }
//
//    fn bubble_up(&mut self, curr: usize) {
//        let mut i = curr;
//        while i > 0 {
//            let parent = Self::parent(i);
//        }
//    }
//}

