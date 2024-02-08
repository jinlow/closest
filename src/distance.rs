use crate::tree::Point;
pub trait DistanceMetric {
    fn distance(&self, p1: &Point, p2: &Point) -> f32;
}

#[derive(Debug, Default)]
pub struct SquaredEuclideanDistance {}

impl DistanceMetric for SquaredEuclideanDistance {
    fn distance(&self, p1: &Point, p2: &Point) -> f32 {
        p1.coordinates
            .iter()
            .zip(&p2.coordinates)
            .map(|(s1, s2)| (s1 - s2).powi(2))
            .sum::<f32>()
    }
}
