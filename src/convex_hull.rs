use std::cmp::Ordering;

pub struct ConvexHull;

#[derive(PartialEq, Debug)]
pub enum Angle {
    CONVEX,
    CONCAVE,
    COLINEAR,
}

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    pub fn equals(&self, point: Self) -> bool {
        (self.x - point.x).abs() < 0.001 &&
        (self.y - point.y).abs() < 0.001
    }
}

impl ConvexHull {
    pub fn orientation(a: &Point, b: &Point, c: &Point) -> Angle {
        let cross_prod = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);
        return match cross_prod.partial_cmp(&0.0).unwrap() {
            Ordering::Less => Angle::CONCAVE,
            Ordering::Equal => Angle::COLINEAR,
            Ordering::Greater => Angle::CONVEX
        }
    }

    pub fn get_angle(anchor: &Point, point: &Point) -> f32 {
        (point.y - anchor.y).atan2(point.x - anchor.x)
    }

    pub fn get_dist(anchor: &Point, point: &Point) -> f32 {
        ((point.x - anchor.x).powi(2) + (point.y - anchor.y).powi(2)).sqrt()
    }

    pub fn generate_hull(points: &Vec<Point>) -> Vec<Point> {
        if points.len() < 3 {
            return Vec::new();
        }
        let anchor = points.iter().min_by(|a, b| {
            (a.y, a.x).partial_cmp(&(b.y, b.x)).unwrap()
        }).expect("Empty list");

        let mut sorted = points.to_vec();
        sorted.sort_by(|a, b| {
            (Self::get_angle(anchor, a), Self::get_dist(anchor, a))
                .partial_cmp(&(Self::get_angle(anchor, b), Self::get_dist(anchor, b)))
                .unwrap()
        });

        let mut hull = sorted[..2].to_vec();

        for i in 2..sorted.len() {
            while Self::orientation(&hull[hull.len()-2], &hull[hull.len()-1], &sorted[i]) != Angle::CONVEX {
                hull.pop();
            }
            hull.push(sorted[i].clone());
        }
        hull.push(anchor.clone());
        hull
    }
}

