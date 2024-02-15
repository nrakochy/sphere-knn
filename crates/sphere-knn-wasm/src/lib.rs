use ::sphere_knn::Opts;
use ::sphere_knn::SphereKnnGetters;
use sphere_knn::sphere_knn;
//use wasm_bindgen::prelude::wasm_bindgen;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SpherableData {
    LatLng { lat: f64, lng: f64 },
    LatLong { lat: f64, long: f64 },
    LatitudeLongitude { latitude: f64, longitude: f64 },
    Position { position: [f64; 2] },
    Location { location: [f64; 2] },
    Geometry { geometry: Geometry },
}

impl SphereKnnGetters for SpherableData {
    fn get_lat(&self) -> f64 {
        match self {
            SpherableData::LatLng { lat, .. } => return *lat,
            SpherableData::LatLong { lat, .. } => return *lat,
            SpherableData::LatitudeLongitude { latitude, .. } => return *latitude,
            SpherableData::Position { position } => return position[0],
            SpherableData::Location { location } => return location[0],
            SpherableData::Geometry { geometry } => return geometry.coordinates[0],
        }
    }
    fn get_lng(&self) -> f64 {
        match self {
            SpherableData::LatLng { lng, .. } => return *lng,
            SpherableData::LatLong { long, .. } => return *long,
            SpherableData::LatitudeLongitude { longitude, .. } => return *longitude,
            SpherableData::Position { position } => return position[1],
            SpherableData::Location { location } => return location[1],
            SpherableData::Geometry { geometry } => return geometry.coordinates[1],
        }
    }
}

fn init(data: JsValue) -> impl Fn(f64, f64, Opts) -> Vec<SpherableData> {
    let value: Vec<SpherableData> = serde_wasm_bindgen::from_value(data).unwrap();
    return sphere_knn::run(value);
}

pub fn run(data: JsValue) -> impl Fn(f64, f64, Opts) -> Vec<SpherableData> {
    return init(data);
}
