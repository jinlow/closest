use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClosestError {
    #[error("All positions must have the same length")]
    DifferingPositionLength,
    #[error("Unable to construct the tree.")]
    UnableToBuildTree,
    #[error("Root node is data.")]
    RootNodeIsData,
}
