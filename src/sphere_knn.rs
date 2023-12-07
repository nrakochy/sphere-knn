use crate::{
    lla_node::{LLANode, Opts},
    tree::get_nearest_neighbors,
    utils::spherical_to_cartesian,
};

fn generate_node<T>(data: T) -> LLANode<T> {
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

fn build<T>(mut nodes: Vec<LLANode<T>>, mut depth: usize) -> Option<&'static LLANode<T>> {
    if nodes.len() <= 1 {
        return nodes.last();
    }

    let axis = depth % nodes[0].position.len();
    nodes.sort_by(|a, b| a.position[axis].partial_cmp(&b.position[axis]).unwrap());
    let median = (nodes.len() as f64 * 0.5).floor() as usize;
    let curr = nodes[depth];
    depth += 1;
    return Some(&LLANode {
        axis,
        split: nodes[median].position[axis] as f64,
        left: Box::new(build(nodes[0..median].to_vec(), depth)),
        right: Box::new(build(nodes[median..].to_vec(), depth)),
        ..curr
    });
}

pub fn build_tree<T>(data: Vec<T>) -> Option<&'static LLANode<&'static T>> {
    // convert data to nodeable
    let mut nodes = data.iter().map(|node| generate_node(node)).collect();
    return build(nodes, 0);
}

pub fn lookup<T>(lat: f64, lng: f64, tree: LLANode<T>, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}
