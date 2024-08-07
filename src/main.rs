mod sequential_relaxer;
mod relaxation_utils;
mod initialiser;
// mod relaxation_problem;

use std::env;

fn main() {

    println!("Initialising.");

    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let array_size: i64 = (&args[1]).parse().unwrap();
    let target_precision: f64 = (&args[2]).parse().unwrap();

    let correct_array: Vec<Vec<i64>> = initialiser::initialise_array(array_size);
    // TODO: Create a context struct. <=== WORK HERE!!!

    println!("Starting rusty relaxation.");

    if sequential_relaxer::relax() {
        println!("Managed to call sequential relaxer!");
    }
}
