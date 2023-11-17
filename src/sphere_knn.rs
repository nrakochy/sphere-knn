use crate::{
    lla_node::{LLANode, Opts},
    tree::get_nearest_neighbors,
    utils::spherical_to_cartesian,
};

fn generate_node<T>(object: T) -> LLANode<T> {
    let lat = 0.0;
    let lng = 0.0;
    let position = spherical_to_cartesian(lat, lng);
    return LLANode {
        position,
        lat,
        lng,
        object,
        split: 0.0,
        axis: 0.0,
        left: Box::new(None),
        right: Box::new(None),
    };
}

pub fn build_tree<T>(nodes: Vec<T>) {
    let _ = nodes.iter().map(|node| generate_node(node));
}

pub fn lookup<T>(lat: f64, lng: f64, tree: LLANode<T>, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}
