use crate::{
    lla_node::NodeOrData, neighbors::get_nearest_neighbors, tree::build_tree,
    utils::spherical_to_cartesian, Data, Opts,
};

use core::fmt::Debug;

fn lookup<T: Clone>(tree: NodeOrData<T>, lat: f64, lng: f64, opts: Opts) -> Vec<T> {
    let position = spherical_to_cartesian(lat, lng);
    return get_nearest_neighbors(position, tree, opts);
}

// Takes the pre-built tree and returns lookup as invokable fn
fn lookup_wrapper<T: Clone>(tree: NodeOrData<T>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    move |lat: f64, lng: f64, opts: Opts| return lookup(tree.clone(), lat, lng, opts)
}

pub fn sphere_knn<T: Clone + Debug>(data: Vec<Data<T>>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    let tree = build_tree(data);
    return lookup_wrapper(tree);
}

// spherekd.lookup(39.95, -75.17, tree, 4).map(function(c) { return c.properties.name }),
// ['New York', 'Troy', 'Boston', 'Miami']

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let ny = Data::new(40.664, -73.939, "New York");
        let philly = Data::new(39.95, -75.17, "Philadelphia");
        let data = vec![
            Data::new(35.690, 139.692, "Tokyo"),
            Data::new(42.358, -71.064, "Boston"),
            Data::new(42.732, -73.693, "Troy"),
            ny,
            Data::new(25.788, -80.224, "Miami"),
            Data::new(51.507, -0.128, "London"),
            Data::new(48.857, 2.351, "Paris"),
            Data::new(48.208, 16.373, "Vienna"),
            Data::new(41.900, 12.500, "Rome"),
            Data::new(39.914, 116.392, "Beijing"),
            Data::new(22.278, 114.159, "Hong Kong"),
            Data::new(37.567, 126.978, "Seoul"),
            Data::new(35.690, 139.692, "Tokyo"),
        ];
        let find_nearest = sphere_knn(data);
        let result = find_nearest(
            philly.lat,
            philly.lng,
            Opts {
                max_distance_threshold: None,
                number_results: Some(1 as usize),
            },
        );
        assert_eq!(result, vec!["New York"]);
    }

    #[test]
    fn test_four_results_correct_order() {
        let ny = Data::new(40.664, -73.939, "New York");
        let philly = Data::new(39.95, -75.17, "Philadelphia");
        let data = vec![
            Data::new(35.690, 139.692, "Tokyo"),
            Data::new(42.358, -71.064, "Boston"),
            Data::new(42.732, -73.693, "Troy"),
            ny,
            Data::new(25.788, -80.224, "Miami"),
            Data::new(51.507, -0.128, "London"),
            Data::new(48.857, 2.351, "Paris"),
            Data::new(48.208, 16.373, "Vienna"),
            Data::new(41.900, 12.500, "Rome"),
            Data::new(39.914, 116.392, "Beijing"),
            Data::new(22.278, 114.159, "Hong Kong"),
            Data::new(37.567, 126.978, "Seoul"),
            Data::new(35.690, 139.692, "Tokyo"),
        ];
        let find_nearest = sphere_knn(data);
        let result = find_nearest(
            philly.lat,
            philly.lng,
            Opts {
                max_distance_threshold: None,
                number_results: Some(4 as usize),
            },
        );
        assert_eq!(result, vec!["New York", "Troy", "Boston", "Miami"]);
    }
}
