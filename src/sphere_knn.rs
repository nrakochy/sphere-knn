use crate::{
    lla_node::{LLANode, Opts, Position},
    tree::get_nearest_neighbors,
    utils::spherical_to_cartesian,
};

fn build<T>(positions: &mut [Option<Position<T>>], mut depth: usize) -> Option<Box<LLANode<T>>> {
    if positions.len() <= 1 {
        return Some(Box::new(LLANode::from_position(positions[0].take())));
    }
    let axis = depth % positions[0].as_ref().unwrap().position.len();
    let median = (positions.len() as f64 * 0.5).floor() as usize;
    let split = positions[median].as_ref().unwrap().position[axis];
    positions.sort_by(|a, b| {
        a.as_ref().unwrap().position[axis]
            .partial_cmp(&b.as_ref().unwrap().position[axis])
            .unwrap()
    });
    depth += 1;
    let (left_pos, right_pos) = positions.split_at_mut(median);
    Some(Box::new(LLANode {
        axis,
        split,
        left: build(left_pos, depth),
        right: build(right_pos, depth),
        position: None,
    }))
}

pub fn build_tree<T>(data: Vec<T>) -> Option<Box<LLANode<T>>> {
    // convert data to nodeable
    let mut positions: Vec<Option<Position<T>>> =
        data.into_iter().map(|d| Some(Position::new(d))).collect();
    build(&mut positions, 0)
}

pub fn lookup<T>(lat: f64, lng: f64, tree: LLANode<T>, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}
