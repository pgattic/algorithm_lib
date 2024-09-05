use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use bitvec::prelude::BitVec;

#[derive(Debug)]
pub struct TreeNode {
    pub content: NodeContent,
    pub count: usize,
}

#[derive(Debug)]
pub enum NodeContent {
    Character(char),
    Children(Box<TreeNode>, Box<TreeNode>),
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &TreeNode) -> bool {
        self.count.eq(&other.count)
    }
}

impl Eq for TreeNode {}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.count.partial_cmp(&self.count)
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl TreeNode {
    pub fn new_leaf(ch: char, count: usize) -> TreeNode {
        TreeNode {
            content: NodeContent::Character(ch),
            count,
        }
    }
    pub fn new_connection(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode {
            count: left.count + right.count,
            content: NodeContent::Children(Box::new(left), Box::new(right)),
        }
    }
}

/// Create a frequency profile for the characters comprising a string of text.
fn profile(text: &str) -> HashMap<char, usize> {
    let mut p = HashMap::new();
    for letter in text.chars() {
        if p.contains_key(&letter) {
            *p.get_mut(&letter).unwrap() += 1;
        } else {
            p.insert(letter, 1);
        }
    }
    p
}

/// Build a binary search tree based on the relative frequency of characters.
/// Keeps the most frequent characters closest to the root.
pub fn build_tree(text: &str) -> TreeNode {
    let profile = self::profile(text);
    let mut pq = BinaryHeap::new();
    for ch in profile.keys() {
        let node = TreeNode::new_leaf(*ch, *profile.get(ch).unwrap());
        pq.push(node);
    }
    while pq.len() > 1 {
        let x = pq.pop().unwrap();
        let y = pq.pop().unwrap();
        let z = TreeNode::new_connection(x, y);
        pq.push(z);
    }
    pq.pop().unwrap()
}

/// Creates a mapping from a character to its corresponding tree traversal.
/// Used in order to reduce the time complexity for the compression algorithm.
/// Time Complexity: `O(n)`, where `n` is the number of unique characters in the tree
pub fn create_encoding_map(root: &TreeNode) -> HashMap<char, BitVec> {
    // The HashMap maps characters to bit vecs
    let mut map = HashMap::new();
    _create_encoding_map(root, BitVec::new(), &mut map);
    map
}

fn _create_encoding_map(node: &TreeNode, code: BitVec, map: &mut HashMap<char, BitVec>) {
    match &node.content {
        NodeContent::Character(ch) => {
            map.insert(*ch, code);
        }
        NodeContent::Children(left_node, right_node) => {
            let mut left_code = code.clone();
            left_code.push(false); // 0
            let mut right_code = code.clone();
            right_code.push(true); // 1
            _create_encoding_map(left_node, left_code, map);
            _create_encoding_map(right_node, right_code, map);
        }
    }
}

/// Encode a string of text into a bit vector according to the encoding map
/// Time Complexity: `O(n)`, where `n` is the length of the string
pub fn encode(text: &str, map: &HashMap<char, BitVec>) -> Option<BitVec> {
    let mut result = BitVec::new();
    for letter in text.chars() {
        if let Some(code) = map.get(&letter) {
            result.append(&mut code.clone());
        } else {
            return None;
        }
    }
    Some(result)
}

/// Decode a huffman-coded bit vector, using a huffman tree
/// Time complexity: `O(n)` where `n` is the length of the BitVec `data`
pub fn decode(data: &BitVec, tree: &TreeNode) -> Option<String> {
    let mut result = String::new();
    let mut curr = tree;
    let mut i = 0;
    while i < data.len() {
        if let NodeContent::Children(left, right) = &curr.content {
            if data[i] { // 1
                curr = right;
            } else { // 0
                curr = left;
            }
            i += 1;
        }
        if let NodeContent::Character(ch) = &curr.content {
            result.push(*ch);
            curr = &tree;
        }
    }
    if curr != tree {
        return None;
    }
    Some(result)
}

