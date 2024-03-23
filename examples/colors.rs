//! An example looking for similar colors

use std::error::Error;

use nearest::{Data, KDTree, Point, SquaredEuclideanDistance};

fn main() -> Result<(), Box<dyn Error>> {
    // RGB color coordinates
    let colors = vec![
        Data::new("blue", vec![0., 0., 255.]),
        Data::new("red", vec![255., 0., 0.]),
        Data::new("navy", vec![17., 4., 89.]),
        Data::new("purple", vec![171., 3., 255.]),
        Data::new("light-blue", vec![61., 118., 224.]),
        Data::new("pink", vec![255., 3., 213.]),
        Data::new("yellow", vec![255., 234., 0.]),
        Data::new("green", vec![16., 145., 25.]),
        Data::new("orange", vec![255., 106., 0.]),
    ];
    // Construct the tree from the vector of data points.
    let tree = KDTree::from_vec(colors, 1).unwrap();
    let point = Point::new(vec![237., 139., 69.]); // Light Orange
    let closest_colors =
        tree.get_nearest_neighbors(&point, 2, &SquaredEuclideanDistance::default());
    println!("The nearest colors to light orange.");
    for color in closest_colors {
        println!("color: {}, squared euclidean distance: {}", color.data, color.distance);
    }
    Ok(())
}
