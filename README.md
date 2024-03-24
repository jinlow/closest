# closest

A simple nearest neighbors implementation in rust.

A rust example, this same example can be run with the following command.
```sh
cargo run --example colors
```

```rust
use std::error::Error;

use closest::{Data, KDTree, Point, SquaredEuclideanDistance};

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

// The nearest colors to light orange.
// color: yellow, squared euclidean distance: 14110
// color: orange, squared euclidean distance: 6174
```

And the equivalent python example.
```python
from nearest import KDTree

colors = [
        ("blue", [0., 0., 255.]),
        ("red", [255., 0., 0.]),
        ("navy", [17., 4., 89.]),
        ("purple", [171., 3., 255.]),
        ("light-blue", [61., 118., 224.]),
        ("pink", [255., 3., 213.]),
        ("yellow", [255., 234., 0.]),
        ("green", [16., 145., 25.]),
        ("orange", [255., 106., 0.]),
    ];
tree = KDTree(colors)
light_orange = [237., 139., 69.]
print(tree.get_nearest_neighbors(light_orange, 2))
#> [(14110.0, 'yellow'), (6174.0, 'orange')]
```
