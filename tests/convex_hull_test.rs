
mod convex_hull_test {
    use algorithm_lib::convex_hull::*;

    #[test]
    pub fn test1_convex_orientation() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(4.0, 0.0);
        let c = Point::new(3.0, 1.0);
        let result = ConvexHull::orientation(&a, &b, &c);
        assert_eq!(result, Angle::CONVEX);
    }
    
    #[test]
    pub fn test2_concave_orientation() {
        let a = Point::new(4.0, 0.0);
        let b = Point::new(3.0, 1.0);
        let c = Point::new(8.0, 8.0);
        let result = ConvexHull::orientation(&a, &b, &c);
        assert_eq!(result, Angle::CONCAVE);
    }
    
    #[test]
    pub fn test3_colinear_orientation() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 1.0);
        let c = Point::new(8.0, 8.0);
        let result = ConvexHull::orientation(&a, &b, &c);
        assert_eq!(result, Angle::COLINEAR);
    }

    #[test]
    pub fn test4_get_angle() {
        let anchor = Point::new(1.0, 2.0);
        let p = Point::new(4.0, 7.0);
        let result = ConvexHull::get_angle(&anchor, &p);
        assert!((result - 1.030).abs() < 0.001); // Equal within 0.001
    }

    #[test]
    pub fn test5_get_distance() {
        let anchor = Point::new(1.0, 2.0);
        let p = Point::new(4.0, 7.0);
        let result = ConvexHull::get_dist(&anchor, &p);
        assert!((result - 5.831).abs() < 0.001);
    }

    #[test]
    pub fn test6_too_few_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
        ];
        let hull = ConvexHull::generate_hull(&points);
        assert_eq!(hull.len(), 0);
    }

    #[test]
    pub fn test7_create_hull() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(3.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(8.0, 8.0),
            Point::new(3.0, 6.0),
            Point::new(1.0, 4.0),
            Point::new(1.0, 3.0),
            Point::new(0.0, 4.0),
            Point::new(0.0, 2.0),
            Point::new(5.5, 7.0)
        ];
        let hull = ConvexHull::generate_hull(&points);

        let expected = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(8.0, 8.0),
            Point::new(3.0, 6.0),
            Point::new(0.0, 4.0),
            Point::new(0.0, 0.0)
        ];

        assert_eq!(hull.len(), expected.len());

        for i in 0..hull.len() {
            assert_eq!(hull[i].x, expected[i].x);
            assert_eq!(hull[i].y, expected[i].y);
        }
    }
}

