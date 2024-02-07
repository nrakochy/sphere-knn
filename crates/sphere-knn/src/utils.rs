use std::f64::consts::PI;

use crate::lla_node::CartesianPosition;

// source (for now): https://github.com/darkskyapp/sphere-knn/blob/master/lib/spherekd.js#L4
const INV_EARTH_DIAMETER_METERS: f64 = 1.0 / (12_742.018 * 1_000.0);
/// Max: will never be exceeded, as distance_from will never be greater than half of this
const EARTH_CIRCUMFERENCE_METERS: f64 = 40_075.017 * 1_000.0;

pub fn spherical_to_cartesian(latitude: f64, longitude: f64) -> CartesianPosition {
    let lat = (latitude * PI) / 180.0;
    let lng = (longitude * PI) / 180.0;
    [lat.cos() * lng.cos(), lat.sin(), lat.cos() * lng.sin()]
}

pub fn convert_max_distance(max: Option<f64>) -> f64 {
    return match max {
        Some(x) if x <= 0.0 => EARTH_CIRCUMFERENCE_METERS,
        Some(x) => (x * INV_EARTH_DIAMETER_METERS).sin() * 2.0,
        None => EARTH_CIRCUMFERENCE_METERS,
    };
}

pub fn make_distance_calculation(a: CartesianPosition, b: CartesianPosition) -> f64 {
    let mut distance = 0.0;

    // CartesianPosition length - 1;
    for i in (0..3).rev() {
        let k = b[i] - a[i];
        distance += k * k;
    }

    return distance;
}
