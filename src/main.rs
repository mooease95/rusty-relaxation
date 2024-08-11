mod sequential_relaxer;
mod relaxation_utils;
mod initialiser;
mod relaxation_context;

use std::env;
use std::time::{Duration, SystemTime};
use crate::relaxation_context::RelaxationContext;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let array_size: usize = (&args[1]).parse().unwrap();
    let target_precision: f64 = (&args[2]).parse().unwrap();
    let debug: bool = (&args[3]).parse().unwrap();

    println!("array_size=[{}], target_precision=[{}].", array_size, target_precision);

    let correct_array: Vec<Vec<f64>> = initialiser::initialise_correct_array(array_size);

    let context = RelaxationContext {
        array_size,
        target_precision,
        correct_array,
        debug
    };

    println!("Starting rusty relaxation.");
    println!("Starting to relax sequentially.");
    let duration_since_epoch_before: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let before_sequential: u128 = duration_since_epoch_before.as_nanos();
    let (successful, steps_taken): (bool, isize) = sequential_relaxer::relax(context);
    let duration_since_epoch_after: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let after_sequential: u128 = duration_since_epoch_after.as_nanos();

    if successful {
        println!("Time taken for a [{}]x[{}] array to relax to a precision of [{}] sequentially = [{}] nanoseconds, number of steps taken = [{}].",
                 array_size, array_size, target_precision, after_sequential - before_sequential, steps_taken);
    }
}
