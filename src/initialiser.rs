
// Assume equation is f(x,y) = x + y
pub fn initialise_correct_array(array_size: usize) -> Vec<Vec<f64>> {
    let mut correct_array: Vec<Vec<f64>> = Vec::with_capacity(array_size);
    for i in 0..array_size {
        let mut correct_array_column: Vec<f64> = Vec::with_capacity(array_size);
        for j in 0..array_size {
            correct_array_column.push(function(i, j));
        }
        correct_array.push(correct_array_column);
    }

    correct_array
}

fn function(x: usize, y: usize) -> f64 {
    // f(x,y) = x + y
    (x + y) as f64
}

pub fn initialise_input_array(array_size: usize) -> Vec<Vec<f64>> {
    let mut input_array: Vec<Vec<f64>> = Vec::with_capacity(array_size);
    for i in 0..array_size {
        let mut input_array_column: Vec<f64> = Vec::with_capacity(array_size);
        for j in 0..array_size {
            // Pad the border with the correct value.
            if i == 0 || j == 0 || i == array_size - 1 || j == array_size - 1 {
                input_array_column.push(function(i, j));
            } else {
                // All other values - values in the middle - should be 0.
                input_array_column.push(0.0);
            }
        }
        input_array.push(input_array_column);
    }

    input_array
}