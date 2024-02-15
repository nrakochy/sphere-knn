use crate::{
    lla_node::{NodeOrData, SphereKnnGetters},
    neighbors::get_nearest_neighbors,
    tree::build_tree,
    utils::spherical_to_cartesian,
    LocationData, Opts,
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

fn shape_data<T: Clone + SphereKnnGetters>(data: Vec<T>) -> Vec<LocationData<T>> {
    return data
        .into_iter()
        .map(|entry| {
            let lat = entry.get_lat();
            let lng = entry.get_lng();
            return LocationData::new(lat, lng, entry);
        })
        .collect();
}

fn init<T: Clone + Debug>(data: Vec<LocationData<T>>) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    let tree = build_tree(data);
    return lookup_wrapper(tree);
}
pub fn sphere_knn<T: Clone + SphereKnnGetters + Debug>(
    data: Vec<T>,
) -> impl Fn(f64, f64, Opts) -> Vec<T> {
    let nodes = shape_data(data);
    return init(nodes);
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'name> SphereKnnGetters for TestData<'name> {
        fn get_lat(&self) -> f64 {
            return self.latitude;
        }
        fn get_lng(&self) -> f64 {
            return self.longitude;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct TestData<'name> {
        latitude: f64,
        longitude: f64,
        name: &'name str,
    }
    const TOKYO: TestData<'_> = TestData {
        latitude: 35.690,
        longitude: 139.692,
        name: "Tokyo",
    };
    const BOSTON: TestData<'_> = TestData {
        latitude: 42.358,
        longitude: -71.064,
        name: "Boston",
    };
    const TROY: TestData<'_> = TestData {
        latitude: 42.732,
        longitude: -73.693,
        name: "Troy",
    };
    const NY: TestData<'_> = TestData {
        latitude: 40.664,
        longitude: -73.939,
        name: "New York",
    };
    const MIAMI: TestData<'_> = TestData {
        latitude: 25.788,
        longitude: -80.224,
        name: "Miami",
    };
    const LONDON: TestData<'_> = TestData {
        latitude: 51.507,
        longitude: -0.128,
        name: "London",
    };
    const PARIS: TestData<'_> = TestData {
        latitude: 48.857,
        longitude: 2.351,
        name: "Paris",
    };
    const VIENNA: TestData<'_> = TestData {
        latitude: 48.208,
        longitude: 16.373,
        name: "Vienna",
    };
    const ROME: TestData<'_> = TestData {
        latitude: 41.900,
        longitude: 12.500,
        name: "Rome",
    };
    const BEIJING: TestData<'_> = TestData {
        latitude: 39.914,
        longitude: 116.392,
        name: "Beijing",
    };
    const HONG_KONG: TestData<'_> = TestData {
        latitude: 22.278,
        longitude: 114.159,
        name: "Hong Kong",
    };
    const SEOUL: TestData<'_> = TestData {
        latitude: 37.567,
        longitude: 126.978,
        name: "Seoul",
    };
    const LOS_ANGELES: TestData<'_> = TestData {
        latitude: 34.0549,
        longitude: -118.2426,
        name: "Los Angeles",
    };
    const MEXICO_CITY: TestData<'_> = TestData {
        latitude: 19.4326,
        longitude: -99.1332,
        name: "Mexico City",
    };

    #[test]
    fn test_one_result() {
        let data = vec![
            TOKYO,
            BOSTON,
            TROY,
            NY,
            MIAMI,
            LONDON,
            PARIS,
            VIENNA,
            ROME,
            BEIJING,
            HONG_KONG,
            SEOUL,
            LOS_ANGELES,
            MEXICO_CITY,
        ];
        let find_nearest = sphere_knn(data);
        let philly = TestData {
            latitude: 39.95,
            longitude: -75.17,
            name: "Philadelphia",
        };
        let result = find_nearest(
            philly.get_lat(),
            philly.get_lng(),
            Opts {
                max_distance_threshold_meters: None,
                number_results: Some(1 as usize),
            },
        );
        assert_eq!(result, vec![NY]);
    }

    #[test]
    fn test_four_results_correct_order_multiple_cities() {
        let data = vec![
            TOKYO,
            BOSTON,
            TROY,
            NY,
            MIAMI,
            LONDON,
            PARIS,
            VIENNA,
            ROME,
            BEIJING,
            HONG_KONG,
            SEOUL,
            LOS_ANGELES,
            MEXICO_CITY,
        ];
        let find_nearest = sphere_knn(data);
        let opts = Opts {
            max_distance_threshold_meters: None,
            number_results: Some(4 as usize),
        };
        let philly = TestData {
            latitude: 39.95,
            longitude: -75.17,
            name: "Philadelphia",
        };
        let result1 = find_nearest(philly.get_lat(), philly.get_lng(), opts);
        //       [vienna, paris, london, rome]
        let berlin = TestData {
            latitude: 52.50,
            longitude: 13.40,
            name: "Berlin",
        };
        let result2 = find_nearest(berlin.get_lat(), berlin.get_lng(), opts);

        let hawaii = TestData {
            latitude: 21.31,
            longitude: -157.80,
            name: "Hawaii",
        };
        let result3 = find_nearest(hawaii.get_lat(), hawaii.get_lng(), opts);
        assert_eq!(result1, vec![NY, TROY, BOSTON, MIAMI]);
        assert_eq!(result2, vec![VIENNA, PARIS, LONDON, ROME]);
        assert_eq!(
            result3,
            vec![
                LOS_ANGELES.clone(),
                MEXICO_CITY.clone(),
                TOKYO.clone(),
                SEOUL.clone()
            ]
        );
    }

    #[test]
    fn test_respects_max_distance() {
        let data = vec![
            TOKYO,
            BOSTON,
            TROY,
            NY,
            MIAMI,
            LONDON,
            PARIS,
            VIENNA,
            ROME,
            BEIJING,
            HONG_KONG,
            SEOUL,
            LOS_ANGELES,
            MEXICO_CITY,
        ];
        let find_nearest = sphere_knn(data);
        let hartford = TestData {
            latitude: 41.76,
            longitude: -72.67,
            name: "hartford",
        };
        let result = find_nearest(
            hartford.get_lat(),
            hartford.get_lng(),
            Opts {
                max_distance_threshold_meters: Some(200_000.0),
                number_results: Some(20 as usize),
            },
        );
        assert_eq!(result, vec![TROY, BOSTON, NY]);
    }
}
