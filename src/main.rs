mod sequential_relaxer;
mod relaxation_utils;
mod initialiser;
mod relaxation_context;

use std::env;
use crate::relaxation_context::RelaxationContext;

fn main() {

    println!("Initialising.");

    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let array_size: i64 = (&args[1]).parse().unwrap();
    let target_precision: f64 = (&args[2]).parse().unwrap();

    let correct_array: Vec<Vec<i64>> = initialiser::initialise_correct_array(array_size);

    let context = RelaxationContext {
        array_size,
        target_precision,
        correct_array
    };

    println!("Starting rusty relaxation.");

    sequential_relaxer::relax(context);
}
