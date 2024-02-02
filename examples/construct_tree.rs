use sphere_knn::{build_tree_from_nodes, Data};

fn main() {
    let data = vec![
        Data::new(42.358, -71.064, "Boston"),
        Data::new(42.732, -73.693, "Troy"),
        Data::new(40.664, -73.939, "New York"),
        Data::new(25.788, -80.224, "Miami"),
        Data::new(51.507, -0.128, "London"),
        Data::new(48.857, 2.351, "Paris"),
        Data::new(48.208, 16.373, "Vienna"),
        Data::new(41.900, 12.500, "Rome"),
        Data::new(39.914, 116.392, "Beijing"),
        Data::new(22.278, 114.159, "Hong Kong"),
        Data::new(37.567, 126.978, "Seoul"),
        Data::new(35.690, 139.692, "Tokyo"),
    ];
    let tree = build_tree_from_nodes(data);
    println!("{:#?}", tree);
}
