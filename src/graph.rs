
#[derive(Clone)]
pub struct Edge {
    pub dest_id: usize,
    pub weight: f64,
}

pub struct Graph {
    graph: Vec<Vec<Edge>>,
    labels: Vec<String>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            graph: vec![Vec::new(); size],
            labels: vec![String::new(); size],
        }
    }

    /// Add a directed edge
    pub fn add_directed_edge(&mut self, src_id: usize, dest_id: usize, weight: f64) {
        let new_edge = Edge{dest_id, weight};
        self.graph[src_id].push(new_edge);
    }
    
    /// Add an undirected edge (two directed edges)
    pub fn add_undirected_edge(&mut self, src_id: usize, dest_id: usize, weight: f64) {
        self.add_directed_edge(src_id, dest_id, weight);
        self.add_directed_edge(dest_id, src_id, weight);
    }

    /// Return the edges of a certain node
    pub fn edges(&self, id: usize) -> &Vec<Edge> {
        &self.graph[id]
    }

    /// Return the number of nodes in the graph
    pub fn size(&self) -> usize {
        self.graph.len()
    }

    /// Set the label of a node in the Graph, as referenced by its id
    pub fn set_label(&mut self, id: usize, label: String) {
        self.labels[id] = label;
    }

    /// Get the label of a node in the Graph, as referenced by its id
    pub fn get_label(&self, id: usize) -> &String {
        &self.labels[id]
    }
}

