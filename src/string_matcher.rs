use core::cmp::min;
use std::collections::HashMap;

pub struct StringMatcher;

impl StringMatcher {

    /// Generates a next-state table based on a `pattern` composed of `inputs` chars
    pub fn build_table(pattern: &str, inputs: &str) -> Vec<HashMap<char, usize>> {
        let mut table = Vec::new();
        for k in 0..pattern.len() + 1 {
            let mut map = HashMap::new();
            for ch in inputs.chars() {
                // Pattern up to K that we match with so far
                let mut pka = pattern[..k].to_string();
                pka.push(ch);
                let mut i = min(k + 1, pattern.len());
                while !pka.ends_with(&pattern[..i]) { i -= 1; }
                map.insert(ch, i);
            }
            table.push(map);
        }
        table
    }

    /// Find matches for `pattern` in `string`, both composed of the characters in `inputs`, Using
    /// the Knuth-Morris-Pratt (KMP) algorithm. Returns the indices where the matches occur.
    pub fn find_matches(text: &str, pattern: &str, inputs: &str) -> Vec<usize> {
        let mut results = Vec::new();
        // No sense searching for a string in a smaller string, or for an empty string
        if text.len() < pattern.len() || pattern.is_empty() {
            return results;
        }

        // Next-state table
        let table = Self::build_table(pattern, inputs);

        // When this value in the table is reached, we have a match:
        let match_state = table.len() - 1;

        let mut state = 0;
        // Traverse the state table using the chars of `text`
        for (i, ch) in text.chars().enumerate() {
            state = *table[state].get(&ch).unwrap();
            if state == match_state {
                results.push(i);
            }
        }
        results
    }
}

