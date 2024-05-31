
use crate::Graph;

pub struct DAG;

/// Directed Acyclic Graph (DAG): Functions that only apply to DAGs
impl DAG {
    /// Get the ids of the Graph's nodes in topological order. Assumes a directed Graph.
    pub fn sort(graph: Graph) -> Vec<usize> {
        // C# says to use a stack for something
        //
        let mut in_order: Vec<usize> = vec![0; graph.size()];
        for i in 0..graph.size() {
            for edge in graph.edges(i) {
                in_order[edge.dest_id] += 1;
            }
        }

        for item in in_order {
            
        }

        //let mut result = vec![];
        //let mut undef_ids: Vec<usize> = (0..graph.size()).collect();
        //while undef_ids.len() > 0 {
        //    let i = undef_ids[0];
        //    let node = graph.edges(i);
        //}
        //result
    }

    /// Get the shortest path to every other accessible node in the graph from the start_vertex
    pub fn shortest_path(graph: Graph, start_vertex: i32) -> (Vec<i32>, Vec<i32>) {
        // Will have to first sort the graph
        let vertices = Self::sort(graph);
        (vec![], vec![])
    }
}

