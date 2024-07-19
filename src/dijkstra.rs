use crate::{graph_heap::GraphHeap, Graph};

pub struct Dijkstra;

impl Dijkstra {
    pub fn shortest_path(g: &Graph, start_vertex: usize) -> Vec<Option<(usize, f32)>> {

        // Initialize Shortest and Predecessor
        let mut result = vec![None; g.size()];
        result[start_vertex] = Some((start_vertex, 0.0));

        // Graph heap will keep the data sorted. Start with the start_vertex at 0 (first element)
        // and the rest of the nodes at infinity (end of the queue)
        let mut q = GraphHeap::new();
        q.enqueue(start_vertex, 0.0);
        for i in 0..g.size() {
            if i != start_vertex {
                q.enqueue(i, std::f32::INFINITY);
            }
        }

        while q.size() > 0 {
            // Pop the item with the smallest total distance from the start_vertex
            let vertex = q.dequeue().unwrap();
            // Relax its edges
            for edge in g.edges(vertex) {
                if let Some((_, alt)) = result[vertex] {
                    let alt = alt + edge.weight;
                    if alt < result[edge.dest_id].unwrap_or((0, std::f32::INFINITY)).1 {
                        result[edge.dest_id] = Some((vertex, alt));
                        q.enqueue(edge.dest_id, alt);
                    }
                }
            }
        }
        result
    }
}

