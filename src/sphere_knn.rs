use crate::{
    lla_node::NodeOrData, neighbors::get_nearest_neighbors, tree::build_tree,
    utils::spherical_to_cartesian, Data, LLANode, Opts,
};

use core::fmt::Debug;

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
