
mod bellman_ford_test {
    use algorithm_lib::{BellmanFord, graph::Graph};
    
    #[test]
    fn test1_no_negative_cycle() {
        let mut g = Graph::new(5);
        g.set_label(0, "s".to_string());
        g.set_label(1, "t".to_string());
        g.set_label(2, "x".to_string());
        g.set_label(3, "y".to_string());
        g.set_label(4, "z".to_string());
        g.add_directed_edge(0, 1, 6.0);
        g.add_directed_edge(0, 3, 7.0);
        g.add_directed_edge(1, 2, 5.0);
        g.add_directed_edge(1, 3, 8.0);
        g.add_directed_edge(1, 4, -4.0);
        g.add_directed_edge(2, 1, -2.0);
        g.add_directed_edge(3, 2, -3.0);
        g.add_directed_edge(3, 4, 9.0);
        g.add_directed_edge(4, 0, 2.0);
        g.add_directed_edge(4, 2, 7.0);
        let result = BellmanFord::shortest_path(&g, 0).unwrap();
        assert_eq!(result[0], Some((0, 0.0)));
        assert_eq!(result[1], Some((2, 2.0)));
        assert_eq!(result[2], Some((3, 4.0)));
        assert_eq!(result[3], Some((0, 7.0)));
        assert_eq!(result[4], Some((1, -2.0)));
    }
    
    #[test]
    fn test2_negative_cycle() {
        let mut g = Graph::new(5);
        g.set_label(0, "s".to_string());
        g.set_label(1, "t".to_string());
        g.set_label(2, "x".to_string());
        g.set_label(3, "y".to_string());
        g.set_label(4, "z".to_string());
        g.add_directed_edge(0, 1, 6.0);
        g.add_directed_edge(0, 3, 7.0);
        g.add_directed_edge(1, 2, 5.0);
        g.add_directed_edge(1, 3, -1.0);
        g.add_directed_edge(1, 4, -4.0);
        g.add_directed_edge(2, 1, -2.0);
        g.add_directed_edge(3, 2, -3.0);
        g.add_directed_edge(3, 4, 9.0);
        g.add_directed_edge(4, 0, 2.0);
        g.add_directed_edge(4, 2, 7.0);
        let result = BellmanFord::shortest_path(&g, 0);
        assert_eq!(result, Err("Graph has a negative weight cycle"));
    }

    //#[test]
    //fn test3() {
    //    let mut g = Graph::new(5);
    //    g.set_label(4, "s".to_string());
    //    g.set_label(1, "t".to_string());
    //    g.set_label(2, "x".to_string());
    //    g.set_label(3, "y".to_string());
    //    g.set_label(0, "z".to_string());
    //    g.add_directed_edge(4, 1, 6.0);
    //    g.add_directed_edge(1, 2, 5.0);
    //    g.add_directed_edge(2, 3, 2.0);
    //    g.add_directed_edge(3, 0, 9.0);
    //    let result = BellmanFord::shortest_path(&g, 4).unwrap();
    //    assert_eq!(result[4], Some((4, 0.0)));
    //    assert_eq!(result[1], Some((4, 6.0)));
    //    assert_eq!(result[2], Some((1, 11.0)));
    //    assert_eq!(result[3], Some((2, 13.0)));
    //    assert_eq!(result[0], Some((3, 22.0)));
    //}
}

