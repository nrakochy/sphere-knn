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

fn build<T>(mut nodes: &[LLANode<T>], mut depth: usize) -> Box<Option<LLANode<T>>> {
    if nodes.len() <= 1 {
        return Box::new(Some(nodes[0]));
    }
    let axis = depth % nodes[0].position.len();
    nodes.sort_by(|a, b| a.position[axis].partial_cmp(&b.position[axis]).unwrap());
    let median = (nodes.len() as f64 * 0.5).floor() as usize;
    let mut curr = nodes[depth];
    depth += 1;
    curr.left = build(&nodes[0..median], depth);
    curr.right = build(&nodes[median..], depth);
    curr.split = nodes[median].position[axis] as f64;
    curr.axis = axis;
    return Box::new(Some(curr))
}

pub fn build_tree<T>(data: Vec<T>) -> Option<LLANode<T>> {
    // convert data to nodeable
    let mut nodes: Vec<LLANode<T>> = data.into_iter().map(|node| generate_node(node)).collect();
    return *build(&nodes, 0);
}

pub fn lookup<T>(lat: f64, lng: f64, tree: LLANode<T>, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}
