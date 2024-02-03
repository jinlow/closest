use crate::error::TreeBuildError;
use std::ops::Range;

/// Points to a node on the node store
/// or data on the data store.
#[derive(Debug)]
pub enum NodeOrDataPointer {
    Node(Node),
    Data(Range<usize>),
}

#[derive(Debug)]
pub struct Node {
    axis: usize,
    split: f32,
    left: Box<NodeOrDataPointer>,
    right: Box<NodeOrDataPointer>,
}

/// Arbitrary data that is queried from n dimensional space.
#[derive(Debug)]
pub struct Data<T> {
    data: T,
    position: Position,
}

impl<T> Data<T> {
    pub fn new(data: T, space: Vec<f32>) -> Self {
        Data {
            data,
            position: Position { space },
        }
    }
}

/// Position defining location in N
/// dimensional space.
#[derive(Debug)]
pub struct Position {
    space: Vec<f32>,
}

impl Position {
    pub fn shape(&self) -> usize {
        self.space.len()
    }
    pub fn point(&self, i: usize) -> f32 {
        self.space[i]
    }
}

/// Tree that is used to partition the data.
#[derive(Debug)]
pub struct KDTree<T> {
    root_node: Node,
    data: Vec<Data<T>>,
}

fn build_tree<T>(
    data: &mut [Data<T>],
    data_location: usize,
    depth: usize,
    position_len: usize,
) -> NodeOrDataPointer {
    if data.len() == 1 {
        return NodeOrDataPointer::Data(data_location..(data_location + data.len()));
    }
    let axis = depth % position_len;
    data.sort_by(|a, b| {
        let a_ = a.position.point(axis);
        let b_ = b.position.point(axis);
        // Consider NaN values Less than everything.
        a_.partial_cmp(&b_).unwrap_or(std::cmp::Ordering::Less)
    });
    let median = data.len() >> 1;
    let node = Node {
        axis,
        split: data[median].position.point(axis),
        left: Box::new(build_tree(
            &mut data[..median],
            data_location,
            depth + 1,
            position_len,
        )),
        right: Box::new(build_tree(
            &mut data[median..],
            data_location + median,
            depth + 1,
            position_len,
        )),
    };
    return NodeOrDataPointer::Node(node);
}

impl<T> KDTree<T> {
    pub fn from_vec(mut data: Vec<Data<T>>) -> Result<Self, TreeBuildError> {
        let position_len = data[0].position.shape();
        let raw_node = build_tree(&mut data, 0, 0, position_len);
        let root_node = match raw_node {
            NodeOrDataPointer::Data(_) => Err(TreeBuildError::UnableToBuildTree),
            NodeOrDataPointer::Node(n) => Ok(n),
        }?;
        Ok(KDTree { root_node, data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_build() {
        // This is a bad example, because this is lat lng, and so our distance
        // measures are not taking the curve of the earth into account, and
        // rather simplifying it to a 2-D plane.
        let data = vec![
            Data::new("Boston", vec![42.358, -71.064]),
            Data::new("Troy", vec![42.732, -73.693]),
            Data::new("New York", vec![40.664, -73.939]),
            Data::new("Miami", vec![25.788, -80.224]),
            Data::new("London", vec![51.507, -0.128]),
            Data::new("Paris", vec![48.857, 2.351]),
            Data::new("Vienna", vec![48.208, 16.373]),
            Data::new("Rome", vec![41.900, 12.500]),
            Data::new("Beijing", vec![39.914, 116.392]),
            Data::new("Hong Kong", vec![22.278, 114.159]),
            Data::new("Seoul", vec![37.567, 126.978]),
            Data::new("Tokyo", vec![35.690, 139.692]),
        ];
        let data_len = data.len();
        let tree = KDTree::from_vec(data).unwrap();
        let mut stack = vec![&tree.root_node];
        let mut found_data = vec![];
        while let Some(node) = stack.pop() {
            match node.left.as_ref() {
                NodeOrDataPointer::Data(i) => found_data.push(i),
                NodeOrDataPointer::Node(n) => stack.push(&n),
            }
            match node.right.as_ref() {
                NodeOrDataPointer::Data(i) => found_data.push(i),
                NodeOrDataPointer::Node(n) => stack.push(n),
            }
        }
        println!("{:#?}", tree);
        assert_eq!(found_data.len(), data_len);

        //assert_eq!(result, 4);
    }
}
