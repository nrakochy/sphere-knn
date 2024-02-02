use std::fmt::Debug;

use crate::{
    lla_node::{Data, LLANode, NodeOrData, Opts},
    tree::get_nearest_neighbors,
    utils::spherical_to_cartesian,
};

// fn generate_node<T: Clone>(data: T) -> LLANode<T> {
//     let lat = 0.0;
//     let lng = 0.0;
//     let position = spherical_to_cartesian(lat, lng);
//     return LLANode {
//         data: Some(Data::new(lat, lng, data)),
//         split: 0.0,
//         axis: 0,
//         left: Box::new(None),
//         right: Box::new(None),
//     };
// }

fn build<T: Clone + Debug>(mut nodes: Vec<Data<T>>, mut depth: usize) -> NodeOrData<T> {
    if nodes.len() == 1 {
        return NodeOrData::Data(nodes[0].clone());
    }

    let axis = depth % nodes[0].position.len();
    nodes.sort_by(|a, b| a.position[axis].partial_cmp(&b.position[axis]).unwrap());
    let median = (nodes.len() as f64 * 0.5).floor() as usize;
    depth += 1;
    return NodeOrData::Node(LLANode {
        axis,
        split: nodes[median].position[axis] as f64,
        left: Box::new(build(nodes[0..median].to_vec(), depth)),
        right: Box::new(build(nodes[median..].to_vec(), depth)),
    });
}

pub fn build_tree_from_nodes<T: Clone + Debug>(nodes: Vec<Data<T>>) -> LLANode<T> {
    match build(nodes, 0) {
        NodeOrData::Data(_) => panic!("Cant happen"),
        NodeOrData::Node(n) => n,
    }
}

pub fn build_tree_from_data<T: Clone + Debug>(data: Vec<T>) -> Option<LLANode<T>> {
    // // convert data to nodeable
    // let nodes = data
    //     .iter()
    //     .cloned()
    //     .map(|node| generate_node(node))
    //     .collect();
    // return build(nodes, 0);
    unimplemented!()
}

fn lookup<T: Clone>(tree: LLANode<T>, lat: f64, lng: f64, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}

fn lookup_wrapper<T: Clone>(tree: LLANode<T>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    move |lat: f64, lng: f64, opts: Opts| return lookup(tree.clone(), lat, lng, opts)
}

pub fn sphere_knn<T: Clone + Debug>(data: Vec<T>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    let maybe_tree = build_tree_from_data(data);
    let tree = maybe_tree.expect("Failed to construct tree. This won't end well");
    return lookup_wrapper(tree);
}
