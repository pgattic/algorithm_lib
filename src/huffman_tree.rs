use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

pub struct TreeNode {
    pub content: NodeContent,
    pub count: usize,
}

pub enum NodeContent {
    Letter(u8),
    Children(Box<TreeNode>, Box<TreeNode>),
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &self::TreeNode) -> bool {
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
    pub fn new_leaf(letter: u8, count: usize) -> TreeNode {
        TreeNode {
            content: NodeContent::Letter(letter),
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

pub fn profile(text: &str) -> HashMap<u8, usize> {
    let mut p = HashMap::new();
    for letter in text.as_bytes() {
        if p.contains_key(letter) {
            *p.get_mut(letter).unwrap() += 1;
        } else {
            p.insert(*letter, 1);
        }
    }
    p
}

pub fn build_tree(profile: HashMap<u8, usize>) -> TreeNode {
    let mut pq = BinaryHeap::new();
    for letter in profile.keys() {
        let node = TreeNode::new_leaf(*letter, *profile.get(letter).unwrap());
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

