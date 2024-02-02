use std::f64::consts::PI;

use crate::lla_node::CartesianPosition;

const INV_EARTH_DIAMETER: f32 = 1.0 / 12742018.0;

pub fn spherical_to_cartesian(lat: f64, lng: f64) -> CartesianPosition {
    let lat = (lat * PI) / 180.0;
    let lng = (lng * PI) / 180.0;
    [lat.cos() * lng.cos(), lat.sin(), lat.cos() * lng.sin()]
}

pub fn get_max_divisor(max_distance: Option<f32>) -> Option<f32> {
    let max: f32 = max_distance.unwrap_or(0.0);
    if max <= 0.0 {
        return None;
    } else {
        let divisor: f32 = max.sin() * 2.0 * INV_EARTH_DIAMETER;
        Some(divisor)
    }
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
