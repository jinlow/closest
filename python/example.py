from nearest import KDTree

points = [
    ("Boston", [42.358, -71.064]),
    ("Troy", [42.732, -73.693]),
    ("New York", [40.664, -73.939]),
    ("Miami", [25.788, -80.224]),
    ("London", [51.507, -0.128]),
    ("Paris", [48.857, 2.351]),
    ("Vienna", [48.208, 16.373]),
    ("Rome", [41.900, 12.500]),
    ("Beijing", [39.914, 116.392]),
    ("Hong Kong", [22.278, 114.159]),
    ("Seoul", [37.567, 126.978]),
    ("Tokyo", [35.690, 139.692]),
]

tree = KDTree(points)
tree.get_nearest_neighbors([43.6766, 4.6278], 2)
