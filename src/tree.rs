use crate::lla_node::{CartesianPosition, LLANode, Opts};

pub fn get_nearest_neighbors<T>(
    position: CartesianPosition,
    tree: LLANode<T>,
    opts: Opts,
) -> Vec<T> {
    return vec![tree.object];
}
