use core::cmp::min;
use std::collections::HashMap;

pub struct StringMatcher;

impl StringMatcher {

    pub fn build_table(pattern: String, inputs: String) -> Vec<HashMap<u8, usize>> {
        let mut table = vec![];
        let pattern_b = pattern.as_bytes();
        let inputs_b = inputs.as_bytes();
        for k in 0..pattern.len()+1 {
            let mut map: HashMap<u8, usize> = HashMap::new();
            for a in inputs_b {
                let mut pka = pattern_b[..k].to_vec();
                pka.push(*a);
                let mut i = min(k+1, pattern.len());
                while !pka.ends_with(&pattern_b[..i]) {i -= 1}
                map.insert(*a, i);
            }
            table.push(map);
        }
        table
    }

    pub fn find_match(text: &str, pattern: String, inputs: String) -> Vec<usize> {
        let table = Self::build_table(pattern, inputs);
        let match_state = table.len()-1;
        let mut results = vec![];
        let mut state = 0;
        let text_b = text.as_bytes();
        for i in 0..text.len() {
            state = *table[state].get(&text_b[i]).unwrap();
            if state == match_state {
                results.push(i);
            }
        }
        results
    }

}

