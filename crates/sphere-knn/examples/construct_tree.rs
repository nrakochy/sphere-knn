use sphere_knn::{build_tree, LocationData};

fn main() {
    let data = vec![
        LocationData::new(42.358, -71.064, "Boston"),
        LocationData::new(42.732, -73.693, "Troy"),
        LocationData::new(40.664, -73.939, "New York"),
        LocationData::new(25.788, -80.224, "Miami"),
        LocationData::new(51.507, -0.128, "London"),
        LocationData::new(48.857, 2.351, "Paris"),
        LocationData::new(48.208, 16.373, "Vienna"),
        LocationData::new(41.900, 12.500, "Rome"),
        LocationData::new(39.914, 116.392, "Beijing"),
        LocationData::new(22.278, 114.159, "Hong Kong"),
        LocationData::new(37.567, 126.978, "Seoul"),
        LocationData::new(35.690, 139.692, "Tokyo"),
    ];
    let tree = build_tree(data);
    println!("{:#?}", tree);
}
