use crate::utils::spherical_to_cartesian;
pub type CartesianPosition = [f64; 3];

#[derive(Debug)]
pub struct Position<T> {
    pub position: CartesianPosition,
    pub lat: f64,
    pub lng: f64,
    /**
     * The generic object payload that holds the consumer's data
     */
    pub data: T,
}

impl<T> Position<T> {
    pub fn new(data: T) -> Self {
        let lat = 0.0;
        let lng = 0.0;
        let position = spherical_to_cartesian(lat, lng);
        Position { position, lat, lng, data }
    }
    
}

#[derive(Debug)]
pub struct LLANode<T> {
    pub axis: usize,
    pub split: f64,
    pub left: Option<Box<LLANode<T>>>,
    pub right: Option<Box<LLANode<T>>>,
    // Leaf data... These are only populated
    // if we are in a leaf...
    pub position: Option<Position<T>>
}

impl <T> LLANode<T> {
    pub fn from_position(position: Option<Position<T>>) -> Self {
        Self { axis: 0, split: 0.0, left: None, right: None, position}
    }
}

pub struct Opts {
    max_distance_threshold: Option<i16>,
    number_results: Option<i16>,
}
