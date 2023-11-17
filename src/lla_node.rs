pub type CartesianPosition = [f64; 3];

#[derive(Debug)]
pub struct LLANode<T> {
    pub position: CartesianPosition,
    pub lat: f64,
    pub lng: f64,
    pub object: T,
    pub axis: f64,
    pub split: f64,
    pub left: Box<Option<LLANode<T>>>,
    pub right: Box<Option<LLANode<T>>>,
}

pub struct Opts {
    max_distance_threshold: Option<i16>,
    number_results: Option<i16>,
}
