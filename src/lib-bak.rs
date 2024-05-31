// Re-export modules
pub mod bellman_ford_shortest_path;
pub mod better_linear_search;
pub mod binary_search;
pub mod convex_hull;
pub mod dag_shortest_path;
pub mod dijkstra_shortest_path;
pub mod graph;
pub mod huffman_tree;
pub mod merge_sort;
pub mod priority_queue;
pub mod quick_sort;
pub mod rsa;
pub mod string_matcher;

// Re-export modules for testing
#[cfg(test)]
mod tests {
    pub mod bellman_ford_shortest_path_test;
    pub mod better_linear_search_test;
    pub mod binary_search_test;
    pub mod convex_hull_test;
    pub mod dag_shortest_path_test;
    pub mod dijkstra_shortest_path_test;
    pub mod huffman_tree_test;
    pub mod merge_sort_test;
    pub mod quick_sort_test;
    pub mod rsa_test;
    pub mod string_matcher_test;
}

