use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreeBuildError {
    #[error("All positions must have the same length")]
    DifferingPositionLength,
    #[error("Unable to construct the tree.")]
    UnableToBuildTree,
}
