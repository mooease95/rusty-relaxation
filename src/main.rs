mod sequential_relaxer;
mod relaxation_utils;
mod initialiser;
mod relaxation_context;
mod concurrent_relaxer;

use std::env;
use std::time::{Duration, SystemTime};
use crate::relaxation_context::RelaxationContext;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let array_size: usize = (&args[1]).parse().unwrap();
    let target_precision: f64 = (&args[2]).parse().unwrap();
    let no_of_threads: usize = (&args[3]).parse().unwrap();
    let debug: bool = (&args[4]).parse().unwrap();

    println!("array_size=[{}], target_precision=[{}], no_of_threds=[{}].", array_size, target_precision, no_of_threads);

    let correct_array: Vec<Vec<f64>> = initialiser::initialise_correct_array(array_size);

    let context = RelaxationContext {
        array_size,
        target_precision,
        correct_array,
        no_of_threads,
        debug
    };

    println!("Starting rusty relaxation.");
    println!("Starting to relax sequentially.");
    let duration_since_epoch_before_sequential: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let before_sequential: u128 = duration_since_epoch_before_sequential.as_nanos();
    let (sequential_successful, sequential_steps_taken): (bool, usize) = sequential_relaxer::relax(&context);
    let duration_since_epoch_after_sequential: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let after_sequential: u128 = duration_since_epoch_after_sequential.as_nanos();

    if sequential_successful {
        println!("Time taken for a [{}]x[{}] array to relax to a precision of [{}] sequentially = [{}] nanoseconds, number of steps taken = [{}].",
                 array_size, array_size, target_precision, after_sequential - before_sequential, sequential_steps_taken);
    }

    println!("Starting to relax concurrently.");
    let duration_since_epoch_before_concurrent: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let before_concurrent: u128 = duration_since_epoch_before_concurrent.as_nanos();
    let (concurrent_successful, concurrent_steps_taken): (bool, usize) = concurrent_relaxer::relax(&context);
    let duration_since_epoch_after_concurrent: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let after_concurrent: u128 = duration_since_epoch_after_concurrent.as_nanos();

    if concurrent_successful {
        println!("Time taken for a [{}]x[{}] array to relax to a precision of [{}] concurrently = [{}] nanoseconds, number of steps taken = [{}].",
                 array_size, array_size, target_precision, after_concurrent - before_concurrent, concurrent_steps_taken);
    }
}
