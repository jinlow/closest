mod distance;
mod error;
mod tree;

pub use crate::tree::KDTree;
pub use crate::distance::{DistanceMetric, SquaredEuclideanDistance};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
