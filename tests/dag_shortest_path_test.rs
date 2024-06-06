
mod dag_shortest_path_test {
    use algorithm_lib::Graph;
    use algorithm_lib::DAG;

    #[test]
    fn topo_sort_test() {
        let mut g = Graph::new(9);
        g.set_label(0, "A".to_string());
        g.set_label(1, "B".to_string());
        g.set_label(2, "C".to_string());
        g.set_label(3, "D".to_string());
        g.set_label(4, "E".to_string());
        g.set_label(5, "F".to_string());
        g.set_label(6, "G".to_string());
        g.set_label(7, "H".to_string());
        g.set_label(8, "I".to_string());
        g.add_directed_edge(0, 3, 3.0);
        g.add_directed_edge(1, 3, 2.0);
        g.add_directed_edge(2, 5, 5.0);
        g.add_directed_edge(3, 6, 9.0);
        g.add_directed_edge(5, 6, 3.0);
        g.add_directed_edge(4, 8, 7.0);
        g.add_directed_edge(6, 8, 7.0);
        g.add_directed_edge(7, 8, 5.0);
        let result = DAG::sort(&g);
        
        assert_eq!(result[0], 7);
        assert_eq!(result[1], 4);
        assert_eq!(result[2], 2);
        assert_eq!(result[3], 5);
        assert_eq!(result[4], 1);
        assert_eq!(result[5], 0);
        assert_eq!(result[6], 3);
        assert_eq!(result[7], 6);
        assert_eq!(result[8], 8);
    }
    
    #[test]
    fn shortest_path_test()
    {
        let mut g = Graph::new(9);
        g.set_label(0, "A".to_string());
        g.set_label(1, "B".to_string());
        g.set_label(2, "C".to_string());
        g.set_label(3, "D".to_string());
        g.set_label(4, "E".to_string());
        g.set_label(5, "F".to_string());
        g.set_label(6, "G".to_string());
        g.set_label(7, "H".to_string());
        g.set_label(8, "I".to_string());
        g.add_directed_edge(0, 3, 3.0);
        g.add_directed_edge(1, 3, 2.0);
        g.add_directed_edge(2, 5, 5.0);
        g.add_directed_edge(3, 6, 9.0);
        g.add_directed_edge(5, 6, 3.0);
        g.add_directed_edge(4, 8, 7.0);
        g.add_directed_edge(6, 8, 7.0);
        g.add_directed_edge(7, 8, 5.0);

        let (pred, distance) = DAG::shortest_path(&g, 2);
        assert_eq!(distance[0], None);
        assert_eq!(distance[1], None);
        assert_eq!(distance[2], Some(0.0));
        assert_eq!(distance[3], None);
        assert_eq!(distance[4], None);
        assert_eq!(distance[5], Some(5.0));
        assert_eq!(distance[6], Some(8.0));
        assert_eq!(distance[7], None);
        assert_eq!(distance[8], Some(15.0));
        assert_eq!(pred[0], None);
        assert_eq!(pred[1], None);
        assert_eq!(pred[2], None);
        assert_eq!(pred[3], None);
        assert_eq!(pred[4], None);
        assert_eq!(pred[5], Some(2));
        assert_eq!(pred[6], Some(5));
        assert_eq!(pred[7], None);
        assert_eq!(pred[8], Some(6));
    }

    #[test]
    fn shortest_path_test_2() {
        let mut g = Graph::new(6);
        g.set_label(0, "r".to_string());
        g.set_label(1, "s".to_string());
        g.set_label(2, "t".to_string());
        g.set_label(3, "x".to_string());
        g.set_label(4, "y".to_string());
        g.set_label(5, "z".to_string());
        g.add_directed_edge(0, 1, 5.0);
        g.add_directed_edge(0, 2, 3.0);
        g.add_directed_edge(1, 2, 2.0);
        g.add_directed_edge(1, 3, 6.0);
        g.add_directed_edge(2, 3, 7.0);
        g.add_directed_edge(2, 4, 4.0);
        g.add_directed_edge(2, 5, 2.0);
        g.add_directed_edge(3, 4, -1.0);
        g.add_directed_edge(3, 5, 1.0);
        g.add_directed_edge(4, 5, -2.0);
        let (pred, distance) = DAG::shortest_path(&g, 1);
        assert_eq!(distance[0], None);
        assert_eq!(distance[1], Some(0.0));
        assert_eq!(distance[2], Some(2.0));
        assert_eq!(distance[3], Some(6.0));
        assert_eq!(distance[4], Some(5.0));
        assert_eq!(distance[5], Some(3.0));
        assert_eq!(pred[0], None);
        assert_eq!(pred[1], None);
        assert_eq!(pred[2], Some(1));
        assert_eq!(pred[3], Some(1));
        assert_eq!(pred[4], Some(3));
        assert_eq!(pred[5], Some(4));
    }
}

