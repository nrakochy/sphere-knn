use crate::utils::spherical_to_cartesian;

pub type CartesianPosition = [f64; 3];

#[derive(Clone, Debug, Default)]
pub struct LLANode<T: Clone> {
    pub position: CartesianPosition,
    pub lat: f64,
    pub lng: f64,
    /**
     * The generic object payload that holds the consumer's data
     */
    pub data: Option<T>,
    pub axis: usize,
    pub split: f64,
    pub left: Box<Option<LLANode<T>>>,
    pub right: Box<Option<LLANode<T>>>,
}

impl<T: Clone> LLANode<T> {
    pub fn new(lat: f64, lng: f64, data: T) -> Self {
        LLANode {
            position: spherical_to_cartesian(lat, lng),
            lat,
            lng,
            data: Some(data),
            axis: 0,
            split: 0.,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

pub struct Opts {
    pub max_distance_threshold: Option<f64>,
    pub number_results: Option<usize>,
}
