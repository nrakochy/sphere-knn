use std::usize;

use crate::{
    lla_node::{CartesianPosition, NodeOrData, Opts},
    utils::{self, convert_max_distance},
};

struct TempPointer<T: Clone> {
    pub node: NodeOrData<T>,
    pub distance_along_axis: f64,
}
struct DataContainerWithDistance<T: Clone> {
    pub data: T,
    pub distance: f64,
}

fn get_threshold(max_distance_threshold: Option<f64>) -> f64 {
    match max_distance_threshold {
        Some(max) => convert_max_distance(max),
        None => f64::MAX,
    }
}

pub fn get_nearest_neighbors<T: Clone>(
    position: CartesianPosition,
    tree: NodeOrData<T>,
    opts: Opts,
) -> Vec<T> {
    let max = get_threshold(opts.max_distance_threshold);
    let num_results: usize = opts.number_results.unwrap_or(usize::MAX);
    let mut result: Vec<DataContainerWithDistance<T>> = vec![];
    // add it to this temp thing to make access easier below
    let entry = TempPointer {
        distance_along_axis: 0.0,
        node: tree,
    };

    let mut stack: Vec<TempPointer<T>> = vec![entry];
    let mut distance_calc: f64;
    let mut node: NodeOrData<T>;
    while stack.len() > 0 {
        if let Some(ptr) = stack.pop() {
            distance_calc = ptr.distance_along_axis;
            node = ptr.node;
        } else {
            continue;
        }

        // too far away already
        if distance_calc > max {
            continue;
        }

        // No need to check this node if further away and num of results achieved already
        if result.len() == num_results
            && result
                .last()
                .is_some_and(|x| x.distance < (distance_calc * distance_calc))
        {
            continue;
        }
        while let NodeOrData::Node(x) = node {
            let lefty = *x.left;
            let righty = *x.right;
            if position[x.axis as usize] < x.split {
                stack.push(TempPointer {
                    node: righty,
                    distance_along_axis: (x.split - position[x.axis as usize]),
                });
                node = lefty;
            } else {
                stack.push(TempPointer {
                    node: lefty,
                    distance_along_axis: position[x.axis as usize] - x.split,
                });
                node = righty;
            }
        }

        // found the edge node
        match node {
            // already exhausted the "Node" check above, this shouldn't ever eval
            NodeOrData::Node(_) => continue,
            NodeOrData::Data(entry) => {
                distance_calc = utils::make_distance_calculation(position, entry.position);

                if distance_calc > (max * max) {
                    continue;
                }

                if result.len() == 0 {
                    result.insert(
                        0,
                        DataContainerWithDistance {
                            distance: distance_calc,
                            data: entry.data,
                        },
                    )
                } else {
                    let insertion_result =
                        result.binary_search_by(|item| item.distance.total_cmp(&distance_calc));
                    let idx = match insertion_result {
                        Ok(ordered) => ordered,
                        Err(not_found_which_is_good) => not_found_which_is_good,
                    };

                    result.insert(
                        idx,
                        DataContainerWithDistance {
                            distance: distance_calc,
                            data: entry.data,
                        },
                    )
                }

                if result.len() > num_results {
                    // remove furthest away when exceeding max result count
                    result.pop();
                }
            }
        }
    }

    return result.iter().map(|x| x.data.clone()).collect::<Vec<_>>();
}
