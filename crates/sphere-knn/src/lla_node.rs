use crate::utils::spherical_to_cartesian;
use positionable::positionable;

pub type CartesianPosition = [f64; 3];

#[derive(Clone, Debug)]
pub enum NodeOrData<T: Clone> {
    Node(LLANode<T>),
    Data(Data<T>),
}

#[derive(Clone, Debug)]
pub struct LLANode<T: Clone> {
    pub axis: usize,
    pub split: f64,
    pub left: Box<NodeOrData<T>>,
    pub right: Box<NodeOrData<T>>,
}

#[positionable]
pub struct Data<T: Clone> {
    pub position: CartesianPosition,
    pub lat: f64,
    pub lng: f64,
    pub data: T,
}

impl<T: Clone> Data<T> {
    pub fn new(lat: f64, lng: f64, data: T) -> Self {
        Data {
            position: spherical_to_cartesian(lat, lng),
            lat,
            lng,
            data,
        }
    }
}

/// Options for filtering results
#[positionable]
pub struct Opts {
    /// Distance in meters to consider in calculation.
    /// Results that exceed this threshold will be omitted
    pub max_distance_threshold_meters: Option<f64>,
    /// Total number of results required.
    /// There is no default, so you likely want to set this
    pub number_results: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: [f64; 2],
}

#[derive(Debug, Clone)]
pub enum SphereKnn<'id> {
    LatLng {
        lat: f64,
        lng: f64,
        id: &'id str,
    },
    LatLong {
        lat: f64,
        long: f64,
        id: &'id str,
    },
    LatitudeLongitude {
        latitude: f64,
        longitude: f64,
        id: &'id str,
    },
    Position {
        position: [f64; 2],
        id: &'id str,
    },
    Location {
        location: [f64; 2],
        id: &'id str,
    },
    Geometry {
        geometry: Geometry,
        id: &'id str,
    },
}

impl<'id> SphereKnn<'id> {
    pub fn data_morph(self) -> Data<String> {
        return match self {
            SphereKnn::LatLng { id, lat, lng } => Data::new(lat, lng, id.to_owned()),
            SphereKnn::LatLong { lat, long, id } => Data::new(lat, long, id.to_owned()),
            SphereKnn::LatitudeLongitude {
                latitude,
                longitude,
                id,
            } => Data::new(latitude, longitude, id.to_owned()),
            SphereKnn::Position { id, position } => {
                Data::new(position[0], position[1], id.to_owned())
            }
            SphereKnn::Location { id, location } => {
                Data::new(location[0], location[1], id.to_owned())
            }
            SphereKnn::Geometry { id, geometry } if geometry.r#type == "geometry" => Data::new(
                // Note well: lat is 2nd, lng is first
                // Need to check if there's a canonical source for this
                geometry.coordinates[1],
                geometry.coordinates[0],
                id.to_owned(),
            ),
            _ => {
                println!(
                    "Failed to parse entry to valid lat/lng. Defaulting to null island {:?}",
                    self
                );
                return Data::new(0.0, 0.0, "NULL ISLAND".to_owned());
            }
        };
    }
}
