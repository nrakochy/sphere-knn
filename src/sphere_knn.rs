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

fn lookup<T: Clone>(tree: LLANode<T>, lat: f64, lng: f64, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}

fn lookup_wrapper<T: Clone>(tree: LLANode<T>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    move |lat: f64, lng: f64, opts: Opts| return lookup(tree.clone(), lat, lng, opts)
}

pub fn sphere_knn<T: Clone>(data: Vec<T>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    let maybe_tree = build_tree(data);
    let tree = match maybe_tree {
        Some(cur) => cur,
        None => panic!("Failed to construct tree. This won't end well"),
    };

    return lookup_wrapper(tree);
}
