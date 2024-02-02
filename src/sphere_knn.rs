use std::fmt::Debug;

use crate::{
    lla_node::{Data, LLANode, NodeOrData, Opts},
    tree::get_nearest_neighbors,
    utils::spherical_to_cartesian,
};

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

pub fn build_tree<T: Clone + Debug>(nodes: Vec<Data<T>>) -> NodeOrData<T> {
    build(nodes, 0)
}

fn lookup<T: Clone>(tree: LLANode<T>, lat: f64, lng: f64, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}

fn lookup_wrapper<T: Clone>(tree: LLANode<T>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    move |lat: f64, lng: f64, opts: Opts| return lookup(tree.clone(), lat, lng, opts)
}

pub fn sphere_knn<T: Clone + Debug>(data: Vec<Data<T>>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    let maybe_tree = build_tree(data);
    let tree = match maybe_tree {
        NodeOrData::Data(_) => panic!("Failed to construct tree. This won't end well"),
        NodeOrData::Node(n) => n,
    };
    return lookup_wrapper(tree);
}
