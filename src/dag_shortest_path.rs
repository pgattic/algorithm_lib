
use crate::Graph;

/// Directed Acyclic Graph (DAG): Functions that only apply to DAGs
pub struct DAG;

impl DAG {
    /// Get the ids of the Graph's nodes in topological order. Assumes a directed Graph.
    /// Time complexity: $O(N)$ where N is the total number of edges
    pub fn sort(graph: &Graph) -> Vec<usize> {
        // Count the references made to each vertex
        let mut ref_count: Vec<usize> = vec![0; graph.size()];
        for i in 0..graph.size() {
            for edge in graph.edges(i) {
                ref_count[edge.dest_id] += 1;
            }
        }

        // Start off with a list of all of the available vertices (those with no incoming references)
        let mut available: Vec<usize> = vec![];
        for (i, item) in ref_count.iter().enumerate() {
            if *item == 0 {
                available.push(i);
            }
        }

        // Transfer all of the available vertices to the result, updating the reference count and
        // availability list
        let mut result = vec![];
        while !available.is_empty() {
            let itm = available.pop().unwrap();
            result.push(itm);
            for edge in graph.edges(itm) {
                ref_count[edge.dest_id] -= 1;
                if ref_count[edge.dest_id] == 0 {
                    available.push(edge.dest_id);
                }
            }
        }

        result
    }

    /// Get the shortest path to every other accessible node in the graph from the start vertex
    /// Time complexity: $O(N)$
    pub fn shortest_path(graph: &Graph, start: usize) -> Vec<Option<(usize, f32)>> {
        // Will have to first topologically sort the graph
        let vert_sort = Self::sort(&graph);

        let mut result = vec![None; graph.size()];
        result[start] = Some((start, 0.0));

        for v in vert_sort {
            if let Some((_v_pred, v_dist)) = result[v] {
                // Relax the vertex
                for edge in graph.edges(v) {
                    if let Some((_e_pred, e_dist)) = result[edge.dest_id] {
                        if edge.weight + v_dist < e_dist {
                            result[edge.dest_id] = Some((v, edge.weight + v_dist));
                        }
                    } else {
                        result[edge.dest_id] = Some((v, edge.weight + v_dist));
                    }
                }
            }
        }
        result
    }
}

