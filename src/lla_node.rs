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

#[derive(Clone, Debug, Default)]
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

pub struct Opts {
    pub max_distance_threshold: Option<f64>,
    pub number_results: Option<usize>,
}
