use crate::utils::spherical_to_cartesian;

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

#[derive(Clone, Debug, Default, Copy)]
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

#[derive(Clone, Debug, Default, Copy)]
/// Options for filtering results
pub struct Opts {
    /// Distance in meters to consider in calculation.
    /// Results that exceed this threshold will be omitted
    pub max_distance_threshold_meters: Option<f64>,
    /// Total number of results required.
    /// There is no default, so you likely want to set this
    pub number_results: Option<usize>,
}
