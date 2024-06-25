use std::cmp::max;

pub struct StringMatcher;

impl StringMatcher {
    fn compute_table(x: &str, y: &str) -> Vec<Vec<usize>> {
        let x = x.as_bytes();
        let y = y.as_bytes();
        let mut table = vec![vec![0; x.len()+1]; y.len()+1];
        for i in 1..=x.len() {
            for j in 1..=y.len() {
                if x[i-1] == y[j-1] {
                    table[i][j] = table[i-1][j-1] + 1;
                } else {
                    table[i][j] = max(table[i][j-1], table[i-1][j]);
                }
            }
        }
        table
    }

    fn lcs_from_table(x: &str, y: &str, table: Vec<Vec<usize>>, i: usize, j: usize) -> String {
        if table[i][j] == 0 {
            return "".to_string();
        }
        let x_chars = x.as_bytes();
        let y_chars = y.as_bytes();
        if x_chars[i-1] == y_chars[j-1] {
            Self::lcs_from_table(x, y, table, i-1, j-1) + &x_chars[i-1].to_string()
        } else {
            if table[i-1][j] > table[i][j-1] {
                Self::lcs_from_table(x, y, table, i-1, j) // left
            } else {
                Self::lcs_from_table(x, y, table, i, j-1) // up
            }
        }
    }

    pub fn find_lcs(x: &str, y: &str) -> String {
        Self::lcs_from_table(x, y, Self::compute_table(x, y), x.len(), y.len())
    }
}

