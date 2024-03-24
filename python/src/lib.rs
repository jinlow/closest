extern crate closest as closest_rust;
use pyo3::prelude::*;

#[derive(FromPyObject, std::cmp::PartialEq, Clone)]
pub enum DataType {
    #[pyo3(transparent, annotation = "str")]
    Str(String),
    #[pyo3(transparent, annotation = "int")]
    Int(i64),
    #[pyo3(transparent, annotation = "int")]
    Flt(f64),
}

// #[pyclass]
// pub struct Data {
//     dt: nearest_rust::Data<DataType>
// }

#[pyclass]
pub struct KDTree {
    tree: closest_rust::KDTree<DataType>,
}

#[pymethods]
impl KDTree {
    /// Instantiate a new KDTree Object.
    #[new]
    #[pyo3(signature = (records, min_points=30))]
    fn new(records: Vec<(DataType, Vec<f32>)>, min_points: usize) -> Self {
        KDTree {
            tree: closest_rust::KDTree::from_iter(
                records
                    .into_iter()
                    .map(|(d, p)| closest_rust::Data::new(d, p)),
                min_points,
            )
            .unwrap(),
        }
    }

    /// Get the K nearest neighbors to a point.
    #[pyo3(signature = (point, k=1))]
    pub fn get_nearest_neighbors(
        &self,
        py: Python,
        point: Vec<f32>,
        k: usize,
    ) -> PyResult<Vec<(f32, PyObject)>> {
        let raw_point = closest_rust::Point::new(point);
        Ok(self
            .tree
            .get_nearest_neighbors(
                &raw_point,
                k,
                &closest_rust::SquaredEuclideanDistance::default(),
            )
            .iter()
            .map(|n| match &n.data {
                DataType::Str(v) => (n.distance, v.into_py(py)),
                DataType::Int(v) => (n.distance, v.into_py(py)),
                DataType::Flt(v) => (n.distance, v.into_py(py)),
            })
            .collect::<Vec<(f32, PyObject)>>())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn closest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KDTree>()?;
    Ok(())
}
