use crate::{
    lla_node::{LLANode, Opts},
    tree::get_nearest_neighbors,
    utils::spherical_to_cartesian,
};

fn generate_node<T: Clone>(data: T) -> LLANode<T> {
    let lat = 0.0;
    let lng = 0.0;
    let position = spherical_to_cartesian(lat, lng);
    return LLANode {
        position,
        lat,
        lng,
        data,
        split: 0.0,
        axis: 0,
        left: Box::new(None),
        right: Box::new(None),
    };
}

fn build<T: Clone>(mut nodes: Vec<LLANode<T>>, mut depth: usize) -> Option<LLANode<T>> {
    if nodes.len() <= 1 {
        return Some(nodes[0].clone());
    }

    let axis = depth % nodes[0].position.len();
    nodes.sort_by(|a, b| a.position[axis].partial_cmp(&b.position[axis]).unwrap());
    let median = (nodes.len() as f64 * 0.5).floor() as usize;
    let curr = nodes[depth].clone();
    depth += 1;
    return Some(LLANode {
        axis,
        split: nodes[median].position[axis] as f64,
        left: Box::new(build(nodes[0..median].to_vec(), depth)),
        right: Box::new(build(nodes[median..].to_vec(), depth)),
        ..curr
    });
}

pub fn build_tree<T: Clone>(data: Vec<T>) -> Option<LLANode<T>> {
    // convert data to nodeable
    let nodes = data
        .iter()
        .cloned()
        .map(|node| generate_node(node))
        .collect();
    return build(nodes, 0);
}

pub fn lookup<T: Clone>(lat: f64, lng: f64, tree: LLANode<T>, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}
