
const NUM_BUCKETS: usize = 256;

/// Custom Hash Map implementation
pub struct HashMap<A> {
    entries: [Vec<(String, A)>; NUM_BUCKETS],
}

impl<A: Clone> HashMap<A> {
    pub fn new() -> Self {
        Self {
            entries: [const { Vec::new() }; NUM_BUCKETS],
        }
    }

    pub fn insert(&mut self, key: String, value: A) {
        let hashed = hash(&key);
        // If pre-existing key, update in place
        for (m_key, m_val) in &mut self.entries[hashed] {
            if *m_key == key {
                *m_val = value;
                return;
            }
        }
        // Else add new entry
        self.entries[hashed].push((key, value))
    }

    pub fn get(&self, key: String) -> Option<A> {
        let hashed = hash(&key);
        for (m_key, m_val) in &self.entries[hashed] {
            if *m_key == key {
                return Some(m_val.clone());
            }
        }
        None
    }

    pub fn remove(&mut self, key: String) -> Option<A> {
        let hashed = hash(&key);
        let bucket = &mut self.entries[hashed];
        for (i, (m_key, _)) in bucket.iter().enumerate() {
            if *m_key == key {
                return Some((*bucket).remove(i).1);
            }
        }
        None
    }
}

fn hash(str: &str) -> usize {
    (str.bytes().fold(0usize, |a, b| a.wrapping_add(b as usize))) % NUM_BUCKETS
}

