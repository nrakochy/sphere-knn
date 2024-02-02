use sphere_knn::{build_tree_from_nodes, LLANode};

fn main() {
    let data = vec![
        LLANode::new(42.358, -71.064, "Boston"),
        LLANode::new(42.732, -73.693, "Troy"),
        LLANode::new(40.664, -73.939, "New York"),
        LLANode::new(25.788, -80.224, "Miami"),
        LLANode::new(51.507, -0.128, "London"),
        LLANode::new(48.857, 2.351, "Paris"),
        LLANode::new(48.208, 16.373, "Vienna"),
        LLANode::new(41.900, 12.500, "Rome"),
        LLANode::new(39.914, 116.392, "Beijing"),
        LLANode::new(22.278, 114.159, "Hong Kong"),
        LLANode::new(37.567, 126.978, "Seoul"),
        LLANode::new(35.690, 139.692, "Tokyo"),
    ];
    let tree = build_tree_from_nodes(data);
    println!("{:?}", tree.unwrap());
}
