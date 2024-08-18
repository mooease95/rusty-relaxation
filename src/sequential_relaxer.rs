
use crate::relaxation_utils as utils;
use crate::relaxation_context::RelaxationContext;
use crate::initialiser;

pub fn relax(context: &RelaxationContext) -> (bool, usize) {
    if context.debug {
        println!("Correct array:");
        for n in 0..context.array_size {
            println!("{:?}", context.correct_array[n]);
        }
    }

    let mut input_array: Vec<Vec<f64>> = initialiser::initialise_input_array(context.array_size);
    let size: usize = context.array_size;

    if context.debug {
        println!("Input array before relaxing:");
        for n in 0..size {
            println!("{:?}", input_array[n]);
        }
    }

    let mut steps_taken: usize = 0;

    let mut needs_another_iteration: bool = true;
    while needs_another_iteration {
        needs_another_iteration = false;
        steps_taken += 1;

        // Copy to a new array.
        let mut new_array_to_relax: Vec<Vec<f64>> = Vec::with_capacity(size);
        new_array_to_relax.extend(input_array.clone());

        if context.debug {
            println!("Working array:");
            for n in 0..size {
                println!("{:?}", new_array_to_relax[n]); // This needs a :? because it's printing an array.
            }
        }

        for row in 1..size-1 {
            for column in 1..size-1 {
                let new_avg_value = utils::average_array(&new_array_to_relax, row, column);
                input_array[row][column] = new_avg_value;
                let precision_reached_for_current_value: bool = utils::check_precision(&context.correct_array, new_avg_value, row, column, context.target_precision);
                if !precision_reached_for_current_value {
                    needs_another_iteration = true;
                }
            }
        }
    }

    if context.debug {
        println!("Input array after relaxing:");
        for n in 0..size {
            println!("{:?}", input_array[n]);
        }
    }

    (true, steps_taken)
}