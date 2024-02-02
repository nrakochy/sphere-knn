use std::f64::MAX;

use crate::{
    lla_node::{CartesianPosition, LLANode, Opts},
    utils,
};

struct TempPointer<T: Clone> {
    pub node: Option<LLANode<T>>,
    pub distance_along_axis: f64,
}
struct DataContainerWithDistance<T: Clone> {
    pub data: T,
    pub distance: f64,
}

pub fn get_nearest_neighbors<T: Clone>(
    position: CartesianPosition,
    tree: LLANode<T>,
    opts: Opts,
) -> Vec<T> {
    unimplemented!()
    // let max = opts.max_distance_threshold.unwrap_or(MAX);
    // let num_results: usize = opts.number_results.unwrap_or(1);
    // let mut result: Vec<DataContainerWithDistance<T>> = vec![];
    // let entry = TempPointer {
    //     node: Some(tree),
    //     distance_along_axis: 0.0,
    // };
    // let mut stack = vec![entry];
    // let mut distance_calc: f64;
    // let mut node: Option<LLANode<T>>;
    // while stack.len() > 0 {
    //     let ptr = stack.pop().unwrap();
    //     distance_calc = ptr.distance_along_axis;
    //     node = ptr.node;

    //     if distance_calc > max {
    //         continue;
    //     }

    //     if result.len() == num_results
    //         && result
    //             .last()
    //             .is_some_and(|x| x.distance < (distance_calc * distance_calc))
    //     {
    //         continue;
    //     }

    //     while node.is_some() {
    //         let x = node.unwrap();
    //         let lefty = *x.left;
    //         let righty = *x.right;
    //         if position[x.axis as usize] < x.split {
    //             stack.push(TempPointer {
    //                 node: righty,
    //                 distance_along_axis: (x.split - position[x.axis as usize]),
    //             });
    //             node = lefty;
    //         } else {
    //             stack.push(TempPointer {
    //                 node: lefty,
    //                 distance_along_axis: position[x.axis as usize] - x.split,
    //             });
    //             node = righty;
    //         }
    //     }

    //     let value = match node {
    //         Some(cur) => cur,
    //         None => continue,
    //     };
    //     distance_calc = utils::make_distance_calculation(position, value.data.unwrap().position);

    //     if distance_calc <= (max * max) {
    //         let idx = result.binary_search_by(|item| item.distance.total_cmp(&distance_calc));
    //         match idx {
    //             Ok(ordered) => result.insert(
    //                 ordered,
    //                 DataContainerWithDistance {
    //                     distance: distance_calc,
    //                     data: value.data.take().data,
    //                 },
    //             ),
    //             Err(e) => println!("something amiss unpacking binary search on result obj: {e:?}"),
    //         }
    //     }

    //     if result.len() > num_results {
    //         // remove furthest away when exceeding max result count
    //         result.pop();
    //     }
    // }

    // return result.iter().map(|x| x.data.clone()).collect::<Vec<_>>();
}
