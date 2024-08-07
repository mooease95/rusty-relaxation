
use crate::relaxation_utils as utils;

pub fn relax() -> bool {
    println!("Starting to relax sequentially.");

    // =============================================
    // Let's start off with a 4x4 array.
    // f(x,y) = x + y
    //
    //      0   1   2   3
    // -------------------
    // 0 |  0   1   2   3
    // 1 |  1   0   0   4
    // 2 |  2   0   0   5
    // 3 |  3   4   5   6
    // =============================================

    let correct_array: [[f64; 4]; 4] = [[0.0, 1.0, 2.0, 3.0], [1.0, 2.0, 3.0, 4.0], [2.0, 3.0, 4.0, 5.0], [3.0, 4.0, 5.0, 6.0]];
    let target_precision: f64 = 0.1;

    println!("Correct array:");
    for n in 0..correct_array.len() {
        println!("{:?}", correct_array[n]);
    }

    println!("Target precision=[{}].", target_precision);

    let mut input_array: [[f64; 4]; 4] = [[0.0, 1.0, 2.0, 3.0], [1.0, 0.0, 0.0, 4.0], [2.0, 0.0, 0.0, 5.0], [3.0, 4.0, 5.0, 6.0]];
    let size: usize = input_array.len();

    println!("Input array before relaxing:");
    for n in 0..size {
        println!("{:?}", input_array[n]);
    }

    let mut steps_taken: i32 = 0;

    let mut needs_another_iteration: bool = true;
    while needs_another_iteration {
        needs_another_iteration = false;
        steps_taken += 1;

        // Copy to a new array.
        let mut new_array_to_relax: [[f64; 4]; 4] = [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]];
        for n in 0..size {
            new_array_to_relax[n] = input_array[n];
        }

        println!("Working array:");
        for n in 0..size {
            println!("{:?}", new_array_to_relax[n]); // This needs a :? because it's printing an array.
        }

        for row in 1..size-1 {
            for column in 1..size-1 {
                let new_avg_value = utils::average_array(new_array_to_relax, row, column);
                input_array[row][column] = new_avg_value;
                let precision_reached_for_current_value: bool = utils::check_precision(correct_array, new_avg_value, row, column, target_precision);
                if !precision_reached_for_current_value {
                    needs_another_iteration = true;
                }
            }
        }
    }
    println!("PRECISION REACHED FOR ALL!! Steps taken=[{}].", steps_taken);

    println!("Input array after relaxing:");
    for n in 0..size {
        println!("{:?}", input_array[n]);
    }
    return true;
}