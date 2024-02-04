use std::fmt::Debug;

use crate::lla_node::{Data, LLANode, NodeOrData};

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
