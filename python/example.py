from closest import KDTree

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
