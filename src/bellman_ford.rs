use crate::Graph;

pub struct BellmanFord;

impl BellmanFord {
    /// Determine the shortest path according to the Bellman-Ford algorithm.
    /// Returns an error if the graph has a negative cycle.
    pub fn shortest_path(g: &Graph, start_vertex: usize) -> Result<Vec<Option<(usize, f32)>>, &'static str> {

        // Initialize Shortest and Predecessor
        let mut result = vec![None; g.size()];
        result[start_vertex] = Some((start_vertex, 0.0));

        // Iterate V-1 times
        // Ensures that the effect of any one node can propagate to the pred value of any other
        for _ in 1..g.size() {
            let mut changes_made = false;
            for v in 0..g.size() {
                if let Some(alt) = result[v] {
                    for e in g.edges(v) {
                        if let Some(comp) = result[e.dest_id] {
                            if alt.1 + e.weight < comp.1 {
                                changes_made = true;
                                result[e.dest_id] = Some((v, alt.1 + e.weight));
                            }
                        } else {
                            changes_made = true;
                            result[e.dest_id] = Some((v, alt.1 + e.weight));
                        }
                    }
                }
            }
            // If no changes were made throughout this iteration, the algorithm can end right now
            if !changes_made {
                break;
            }
        }

        // Check for negative weight cycles
        for v in 0..g.size() {
            if let Some(path) = result[v] {
                for e in g.edges(v) {
                    if path.1 + e.weight < result[e.dest_id].unwrap().1 {
                        return Err("Graph has a negative weight cycle");
                    } 
                }
            }
        }

        Ok(result)
    }
}

