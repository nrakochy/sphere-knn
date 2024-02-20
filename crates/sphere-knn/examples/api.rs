use sphere_knn::{run, Opts, SphereKnnGetters};

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

impl<'name> SphereKnnGetters for TestData<'name> {
    fn get_lat(&self) -> f64 {
        return self.latitude;
    }
    fn get_lng(&self) -> f64 {
        return self.longitude;
    }
}

fn main() {
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

    print!("\nBuilding a tree for 14 cities...\n-----------------------------------\n");
    let find_nearest = run(data);
    let hartford = TestData {
        latitude: 41.76,
        longitude: -72.67,
        name: "hartford",
    };

    println!(
        "Looking for closest cities to hartford within 200km:\n {:#?}\n-----------------------------------\n",
        hartford
    );
    let result = find_nearest(
        hartford.get_lat(),
        hartford.get_lng(),
        Opts {
            max_distance_threshold_meters: Some(200_000.0),
            number_results: Some(20 as usize),
        },
    );
    println!("{:#?}\n-----------------------------------\n", result);
}
