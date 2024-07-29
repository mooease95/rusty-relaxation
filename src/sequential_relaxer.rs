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

    let input_array: [[i32; 4]; 4] = [[0, 1, 2, 3], [1, 0, 0, 4], [2, 0, 0, 5], [3, 4, 5, 6]];
    let size: usize = input_array.len();
    println!("Size of array is={}.", size);

    let mut steps_taken: i32 = 0;

    let mut needs_another_iteration: bool = true;
    while needs_another_iteration {
        needs_another_iteration = false;
        steps_taken += 1;

        println!("Input array:");
        for n in 0..size {
            println!("{:?}", input_array[n]);
        }

        // Copy to a new array.
        let mut new_array_to_relax: [[i32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        for n in 0..size {
            new_array_to_relax[n] = input_array[n];
        }

        println!("Working array:");
        for n in 0..size {
            println!("{:?}", new_array_to_relax[n]); // This needs a :? because it's printing an array.
        }

        for row in 0..size {
            for column in 0..size {
                println!("{}", new_array_to_relax[row][column]) // This only prints a number so does not need a :?.
                // TODO: Do the average here and continue. <=== WORK HERE!!!
            }
        }
    }


    return true;
}