
use crate::Graph;

/// Directed Acyclic Graph (DAG): Functions that only apply to DAGs
pub struct DAG;

impl DAG {
    /// Get the ids of the Graph's nodes in topological order. Assumes a directed Graph.
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
    pub fn shortest_path(graph: &Graph, start: usize) -> (Vec<Option<usize>>, Vec<Option<f64>>) {
        // Will have to first sort the graph
        let vert_sort = Self::sort(&graph);

        // The item order of these two is to remain consistent
        let mut shortest = vec![None; graph.size()];
        let mut pred = vec![None; graph.size()];
        shortest[start] = Some(0.0);
        for v in vert_sort {
            if let Some(v_dist) = shortest[v] {
                for edge in graph.edges(v) {
                    if let Some(e_dist) = shortest[edge.dest_id] {
                        if edge.weight + v_dist < e_dist {
                            shortest[edge.dest_id] = Some(edge.weight + v_dist);
                            pred[edge.dest_id] = Some(v);
                        }
                    } else {
                        shortest[edge.dest_id] = Some(edge.weight + v_dist);
                        pred[edge.dest_id] = Some(v);
                    }
                }
            }
        }
        (pred, shortest)
    }
}

