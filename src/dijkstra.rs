use crate::{graph_heap::GraphHeap, Graph};

pub struct Dijkstra;

impl Dijkstra {
    pub fn shortest_path(g: &Graph, start_vertex: usize) -> (Vec<Option<f32>>, Vec<Option<usize>>) {

        // Initialize Shortest and Predecessor
        let mut shortest = vec![None; g.size()];
        let mut pred = vec![None; g.size()];
        shortest[start_vertex] = Some(0.0);

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
            let u = q.dequeue().unwrap();
            // Relax its edges
            for edge in g.edges(u) {
                if let Some(alt) = shortest[u] {
                    let alt = alt + edge.weight;
                    if alt < shortest[edge.dest_id].unwrap_or(std::f32::INFINITY) {
                        shortest[edge.dest_id] = Some(alt);
                        pred[edge.dest_id] = Some(u);
                        q.enqueue(edge.dest_id, alt);
                    }
                }
            }
        }
        (shortest, pred)
    }
}

