use crate::{graph_heap::GraphHeap, Graph};

pub struct DijkstraShortestPath;

impl DijkstraShortestPath {
    pub fn shortest_path(g: &Graph, start_vertex: usize) -> (Vec<Option<f32>>, Vec<Option<usize>>) {
        let mut shortest = vec![None; g.size()];
        let mut pred = vec![None; g.size()];
        shortest[start_vertex] = Some(0.0);
        let mut q = GraphHeap::new();
        q.enqueue(start_vertex, 0.0);
        for i in 0..g.size() {
            if i != start_vertex {
                q.enqueue(i, std::f32::INFINITY);
            }
        }
        while q.size() > 0 {
            let u = q.dequeue().unwrap();
            for edge in g.edges(u) {
                let alt = shortest[u].unwrap_or(std::f32::INFINITY) + edge.weight;
                if alt < shortest[edge.dest_id].unwrap_or(std::f32::INFINITY) {
                    shortest[edge.dest_id] = Some(alt);
                    pred[edge.dest_id] = Some(u);
                    q.enqueue(edge.dest_id, alt);
                }
            }
        }
        (shortest, pred)
    }
}

