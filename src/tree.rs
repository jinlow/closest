use crate::distance::DistanceMetric;
use crate::error::TreeBuildError;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Points to a node on the node store
/// or data on the data store.
#[derive(Debug)]
pub enum NodeOrDataPointer {
    Node(Node),
    Data((usize, usize)),
}

#[derive(Debug)]
pub struct Node {
    axis: usize,
    data_pointer: usize,
    left: Box<NodeOrDataPointer>,
    right: Box<NodeOrDataPointer>,
}

/// Arbitrary data that is queried from n dimensional coordinates.
#[derive(Debug)]
pub struct Data<T> {
    data: T,
    point: Point,
}

impl<T> Data<T> {
    pub fn new(data: T, coordinates: Vec<f32>) -> Self {
        Data {
            data,
            point: Point { coordinates },
        }
    }
}

/// Point defining location in N
/// dimensional coordinates.
#[derive(Debug)]
pub struct Point {
    pub coordinates: Vec<f32>,
}

impl Point {
    pub fn shape(&self) -> usize {
        self.coordinates.len()
    }
    pub fn point(&self, i: usize) -> f32 {
        self.coordinates[i]
    }
}

struct RawNeighbor {
    distance: f32,
    data_pointer: usize,
}

impl RawNeighbor {
    pub fn new(distance: f32, data_pointer: usize) -> Self {
        RawNeighbor {
            distance,
            data_pointer,
        }
    }
}

/// Reversing, to make BinaryHeap Minimum
impl Ord for RawNeighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

impl PartialOrd for RawNeighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RawNeighbor {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for RawNeighbor {}

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
    point_len: usize,
) -> NodeOrDataPointer {
    // Only can split further if there is at least 3 records
    if data.len() < 3 {
        return NodeOrDataPointer::Data((data_location, (data_location + data.len())));
    }
    let axis = depth % point_len;
    data.sort_by(|a, b| {
        let a_ = a.point.point(axis);
        let b_ = b.point.point(axis);
        // Consider NaN values Less than everything.
        a_.partial_cmp(&b_).unwrap_or(std::cmp::Ordering::Less)
    });
    let median = data.len() >> 1;
    let node = Node {
        axis,
        data_pointer: median + data_location,
        left: Box::new(build_tree(
            &mut data[..median],
            data_location,
            depth + 1,
            point_len,
        )),
        right: Box::new(build_tree(
            &mut data[(median + 1)..],
            data_location + median + 1,
            depth + 1,
            point_len,
        )),
    };
    return NodeOrDataPointer::Node(node);
}

impl<T> KDTree<T> {
    pub fn from_vec(mut data: Vec<Data<T>>) -> Result<Self, TreeBuildError> {
        let point_len = data[0].point.shape();
        let raw_node = build_tree(&mut data, 0, 0, point_len);
        let root_node = match raw_node {
            NodeOrDataPointer::Data(_) => Err(TreeBuildError::UnableToBuildTree),
            NodeOrDataPointer::Node(n) => Ok(n),
        }?;
        Ok(KDTree { root_node, data })
    }
    fn get_data(&self, data_idx: usize) -> &Data<T> {
        &self.data[data_idx]
    }
    fn get_data_point(&self, data_idx: usize) -> &Point {
        &self.get_data(data_idx).point
    }

    fn nearest_neighbors<D: DistanceMetric>(
        &self,
        k: usize,
        point: &Point,
        node: &NodeOrDataPointer,
        depth: usize,
        heap: &mut BinaryHeap<RawNeighbor>,
        distance_metric: D,
    ) {
        match node {
            NodeOrDataPointer::Node(n) => {
                let distance =
                    distance_metric.distance(&point, self.get_data_point(n.data_pointer));
                match heap.peek() {
                    None => heap.push(RawNeighbor::new(distance, n.data_pointer)),
                    Some(worst_neighbor) => {
                        if k < heap.len() || distance < worst_neighbor.distance {
                            heap.push(RawNeighbor::new(distance, n.data_pointer))
                        }
                    }
                }
                // TODO: Add visit children logic here.
            }
            NodeOrDataPointer::Data((start, stop)) => {
                let neighbor_candidates = (*start..*stop)
                    .map(|data_pointer| {
                        Reverse(RawNeighbor::new(
                            distance_metric.distance(&point, self.get_data_point(data_pointer)),
                            data_pointer,
                        ))
                    })
                    .collect::<BinaryHeap<Reverse<RawNeighbor>>>();
                if heap.len() == 0 {
                    // Push k records onto the heap and don't worry about anything.
                    neighbor_candidates
                        .into_iter()
                        .take(k)
                        .for_each(|n| heap.push(n.0));
                } else {
                    if let Some(b) = heap.pop() {
                        // b is the nearest neighbor with the greatest
                        // distance, if neighbor_candidates has a smaller distance
                        // add it and keep going
                    }
                }
            }
        }
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
        let mut found_data = vec![tree.root_node.data_pointer..(tree.root_node.data_pointer + 1)];
        while let Some(node) = stack.pop() {
            match node.left.as_ref() {
                NodeOrDataPointer::Data((start, stop)) => found_data.push(*start..*stop),
                NodeOrDataPointer::Node(n) => {
                    stack.push(&n);
                    found_data.push(n.data_pointer..(n.data_pointer + 1));
                }
            }
            match node.right.as_ref() {
                NodeOrDataPointer::Data((start, stop)) => found_data.push(*start..*stop),
                NodeOrDataPointer::Node(n) => {
                    stack.push(&n);
                    found_data.push(n.data_pointer..(n.data_pointer + 1));
                }
            }
        }
        println!("{:#?}", tree);
        let mut data_idx = Vec::new();
        for g in found_data {
            for i in g {
                data_idx.push(i);
            }
        }
        data_idx.sort();
        println!("{:?}", data_idx);
        assert_eq!(data_idx.len(), data_len);
        let expected_idx: Vec<usize> = (0..tree.data.len()).collect();
        assert_eq!(expected_idx, data_idx);

        //assert_eq!(result, 4);
    }
}
