mod distance;
mod error;
mod tree;

pub use crate::tree::{KDTree, Data, Point};
pub use crate::distance::{DistanceMetric, SquaredEuclideanDistance};
