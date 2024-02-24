use crate::distance::DistanceMetric;
use crate::error::NearestError;
use std::cmp::Ordering;
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
    data_pointer: usize,
    left: Box<NodeOrDataPointer>,
    right: Box<NodeOrDataPointer>,
}

/// Arbitrary data that is queried from n dimensional coordinates.
#[derive(Debug)]
pub struct Data<T: Clone> {
    data: T,
    point: Point,
}

impl<T: Clone> Data<T> {
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
    pub fn new(coordinates: Vec<f32>) -> Self {
        Point { coordinates }
    }
}

impl Point {
    pub fn shape(&self) -> usize {
        self.coordinates.len()
    }
    pub fn point(&self, i: usize) -> f32 {
        self.coordinates[i]
    }
}

#[derive(Debug)]
pub struct Neighbor<T: Clone> {
    pub distance: f32,
    pub data: T,
}

impl<T: Clone> Ord for Neighbor<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

impl<T: Clone> PartialOrd for Neighbor<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone> PartialEq for Neighbor<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T: Clone> Eq for Neighbor<T> {}

#[derive(Debug)]
struct RawNeighbor {
    distance: f32,
    data_pointer: usize,
}

impl RawNeighbor {
    pub fn as_neighbor<T: Clone>(self, data: &[Data<T>]) -> Neighbor<T> {
        Neighbor {
            distance: self.distance,
            data: data[self.data_pointer].data.clone(),
        }
    }
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
pub struct KDTree<T: Clone> {
    root_node: NodeOrDataPointer,
    data: Vec<Data<T>>,
    dimension: usize,
}

fn build_tree<T: Clone>(
    data: &mut [Data<T>],
    data_location: usize,
    depth: usize,
    point_len: usize,
    min_points: usize,
) -> NodeOrDataPointer {
    // Only can split further if there is at least 3 records
    if (data.len() < min_points) || (data.len() < 3) {
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
        data_pointer: median + data_location,
        left: Box::new(build_tree(
            &mut data[..median],
            data_location,
            depth + 1,
            point_len,
            min_points,
        )),
        right: Box::new(build_tree(
            &mut data[(median + 1)..],
            data_location + median + 1,
            depth + 1,
            point_len,
            min_points,
        )),
    };
    return NodeOrDataPointer::Node(node);
}

impl<T: Clone> KDTree<T> {
    pub fn from_iter<I: Iterator<Item = Data<T>>>(
        data: I,
        min_points: usize,
    ) -> Result<Self, NearestError> {
        Self::from_vec(data.collect(), min_points)
    }
    pub fn from_vec(mut data: Vec<Data<T>>, min_points: usize) -> Result<Self, NearestError> {
        let point_len = data[0].point.shape();
        let root_node = build_tree(&mut data, 0, 0, point_len, min_points);
        Ok(KDTree {
            root_node,
            data,
            dimension: point_len,
        })
    }
    pub fn get_root_node(&self) -> Result<&Node, NearestError> {
        match &self.root_node {
            NodeOrDataPointer::Data(_) => Err(NearestError::RootNodeIsData),
            NodeOrDataPointer::Node(n) => Ok(&n),
        }
    }
    fn get_data(&self, data_idx: usize) -> &Data<T> {
        &self.data[data_idx]
    }
    fn get_data_point(&self, data_idx: usize) -> &Point {
        &self.get_data(data_idx).point
    }
    pub fn get_nearest_neighbors<D: DistanceMetric>(
        &self,
        point: &Point,
        k: usize,
        distance_metric: &D,
    ) -> Vec<Neighbor<T>> {
        let mut heap = BinaryHeap::new();
        self.nearest_neighbors(point, k, &self.root_node, 0, &mut heap, distance_metric);
        heap.into_iter()
            .map(|r| r.as_neighbor(&self.data))
            .collect()
    }
    fn nearest_neighbors<D: DistanceMetric>(
        &self,
        point: &Point,
        k: usize,
        node: &NodeOrDataPointer,
        depth: usize,
        heap: &mut BinaryHeap<RawNeighbor>,
        distance_metric: &D,
    ) {
        match node {
            NodeOrDataPointer::Node(n) => {
                let distance =
                    distance_metric.distance(&point, self.get_data_point(n.data_pointer));
                match heap.peek() {
                    None => heap.push(RawNeighbor::new(distance, n.data_pointer)),
                    Some(worst_neighbor) => {
                        if distance < worst_neighbor.distance {
                            if heap.len() >= k {
                                heap.pop();
                            }
                            heap.push(RawNeighbor::new(distance, n.data_pointer))
                        }
                    }
                }
                let axis = depth % self.dimension;
                let diff =
                    point.coordinates[axis] - self.get_data_point(n.data_pointer).coordinates[axis];
                let (close, away) = if diff <= 0. {
                    (n.left.as_ref(), n.right.as_ref())
                } else {
                    (n.right.as_ref(), n.left.as_ref())
                };
                self.nearest_neighbors(point, k, close, depth + 1, heap, distance_metric);
                if let Some(worst_neighbor) = heap.peek() {
                    if diff.powi(2) < worst_neighbor.distance {
                        self.nearest_neighbors(point, k, away, depth + 1, heap, distance_metric);
                    }
                }
            }
            NodeOrDataPointer::Data((start, stop)) => {
                let mut neighbor_candidates = (*start..*stop)
                    .map(|data_pointer| {
                        RawNeighbor::new(
                            distance_metric.distance(&point, self.get_data_point(data_pointer)),
                            data_pointer,
                        )
                    })
                    .collect::<Vec<RawNeighbor>>();
                // Add all candidates if we have enough space.
                if k.saturating_sub(heap.len()) >= neighbor_candidates.len() {
                    heap.extend(neighbor_candidates)
                } else {
                    // Sort in reverse order.
                    neighbor_candidates.sort_unstable_by(|a, b| b.cmp(a));
                    loop {
                        match neighbor_candidates.pop() {
                            None => break,
                            Some(best_candidate) => {
                                if heap.len() < k {
                                    heap.push(best_candidate)
                                } else {
                                    if let Some(worst_neighbor) = heap.peek() {
                                        if worst_neighbor > &best_candidate {
                                            heap.pop();
                                            heap.push(best_candidate)
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distance::SquaredEuclideanDistance;

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
        let tree = KDTree::from_vec(data, 1).unwrap();
        let mut stack = vec![tree.get_root_node().unwrap()];
        let mut found_data = vec![
            tree.get_root_node().unwrap().data_pointer
                ..(tree.get_root_node().unwrap().data_pointer + 1),
        ];
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
        // println!("{:#?}", tree);
        let mut data_idx = Vec::new();
        for g in found_data {
            for i in g {
                data_idx.push(i);
            }
        }
        data_idx.sort();
        // println!("{:?}", data_idx);
        assert_eq!(data_idx.len(), data_len);
        let expected_idx: Vec<usize> = (0..tree.data.len()).collect();
        assert_eq!(expected_idx, data_idx);

        // Get nearest neighbor
        let point = Point::new(vec![43.6766, 4.6278]); // Arles
        let nearest = tree.get_nearest_neighbors(&point, 1, &SquaredEuclideanDistance::default());
        assert_eq!(nearest[0].data, "Paris");
    }
}
