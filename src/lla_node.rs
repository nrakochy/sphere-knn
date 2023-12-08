pub type CartesianPosition = [f64; 3];

#[derive(Clone, Debug)]
pub struct LLANode<T: Clone> {
    pub position: CartesianPosition,
    pub lat: f64,
    pub lng: f64,
    /**
     * The generic object payload that holds the consumer's data
     */
    pub data: T,
    pub axis: usize,
    pub split: f64,
    pub left: Box<Option<LLANode<T>>>,
    pub right: Box<Option<LLANode<T>>>,
}

pub struct Opts {
    max_distance_threshold: Option<i16>,
    number_results: Option<i16>,
}
