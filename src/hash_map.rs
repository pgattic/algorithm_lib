
/// Custom Hash Map implementation
pub struct HashMap<A> {
    entries: [Vec<(String, A)>; 256],
}

impl<A: Clone> HashMap<A> {
    pub fn new() -> Self {
        Self {
            entries: [const { Vec::new() }; 256],
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
        for (i, (m_key, m_val)) in (&self.entries[hashed].clone()).iter().enumerate() {
            if *m_key == key {
                let _ = &self.entries[hashed].remove(i);
                return Some(m_val.clone());
            }
        }
        None
    }
}

fn hash(str: &str) -> usize {
    str.bytes().sum::<u8>() as usize % 256
}

