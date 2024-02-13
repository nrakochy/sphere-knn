use crate::utils::spherical_to_cartesian;
use positionable::positionable;

pub type CartesianPosition = [f64; 3];

pub trait SphereKnnGetters {
    fn get_lat(&self) -> f64;
    fn get_lng(&self) -> f64;
}

#[derive(Clone, Debug)]
pub enum NodeOrData<T: Clone> {
    Node(LLANode<T>),
    Data(LocationData<T>),
}

#[derive(Clone, Debug)]
pub struct LLANode<T: Clone> {
    pub axis: usize,
    pub split: f64,
    pub left: Box<NodeOrData<T>>,
    pub right: Box<NodeOrData<T>>,
}

#[positionable]
pub struct LocationData<T: Clone> {
    pub position: CartesianPosition,
    pub lat: f64,
    pub lng: f64,
    pub data: T,
}

impl<T: Clone> LocationData<T> {
    pub fn new(lat: f64, lng: f64, data: T) -> Self {
        LocationData {
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
