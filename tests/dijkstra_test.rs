
mod dijkstra_test {
    use algorithm_lib::{Dijkstra, graph::Graph};

    #[test]
    fn test1_graph1() {
        let mut g = Graph::new(5);
        g.set_label(0, "s".to_string());
        g.set_label(1, "t".to_string());
        g.set_label(2, "x".to_string());
        g.set_label(3, "y".to_string());
        g.set_label(4, "z".to_string());
        g.add_directed_edge(0, 1, 6.0);
        g.add_directed_edge(0, 3, 4.0);
        g.add_directed_edge(1, 2, 3.0);
        g.add_directed_edge(1, 3, 2.0);
        g.add_directed_edge(2, 4, 4.0);
        g.add_directed_edge(3, 1, 1.0);
        g.add_directed_edge(3, 2, 9.0);
        g.add_directed_edge(3, 4, 3.0);
        g.add_directed_edge(4, 2, 5.0);
        g.add_directed_edge(4, 0, 7.0);
        let result = Dijkstra::shortest_path(&g, 1);
        assert_eq!(result[0], Some((4, 12.0)));
        assert_eq!(result[1], Some((1, 0.0)));
        assert_eq!(result[2], Some((1, 3.0)));
        assert_eq!(result[3], Some((1, 2.0)));
        assert_eq!(result[4], Some((3, 5.0)));
    }
    
    #[test]
    fn test2_graph2() {
        let mut g = Graph::new(5);
        g.set_label(0, "A".to_string());
        g.set_label(1, "B".to_string());
        g.set_label(2, "C".to_string());
        g.set_label(3, "D".to_string());
        g.set_label(4, "E".to_string());
        g.add_directed_edge(0, 1, 4.0);
        g.add_directed_edge(0, 3, 7.0);
        g.add_directed_edge(1, 0, 3.0);
        g.add_directed_edge(1, 2, 6.0);
        g.add_directed_edge(2, 1, 6.0);
        g.add_directed_edge(2, 4, 3.0);
        g.add_directed_edge(3, 2, 1.0);
        g.add_directed_edge(4, 0, 4.0);
        let result = Dijkstra::shortest_path(&g, 0);
        assert_eq!(result[0], Some((0, 0.0)));
        assert_eq!(result[1], Some((0, 4.0)));
        assert_eq!(result[2], Some((3, 8.0)));
        assert_eq!(result[3], Some((0, 7.0)));
        assert_eq!(result[4], Some((2, 11.0)));
    }
}

