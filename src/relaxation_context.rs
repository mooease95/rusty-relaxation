

pub struct RelaxationContext {
    pub array_size: usize,
    pub target_precision: f64,
    pub correct_array: Vec<Vec<f64>>,
    pub no_of_threads: usize,
    pub debug: bool
}