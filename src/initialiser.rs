
// Assume equation is f(x,y) = x + y
// TODO: Can I make the vector have f64?
pub fn initialise_correct_array(array_size: i64) -> Vec<Vec<i64>> {
    let mut correct_array: Vec<Vec<i64>> = Vec::with_capacity(usize::from(array_size));
    for i in 0..array_size {
        for j in 0..array_size {
            correct_array[i][j] = function(i, j);
        }
    }

    return correct_array;
}

fn function(x: i64, y: i64) -> i64 {
    // f(x,y) = x + y
    return x + y;
}

pub fn initialise_input_array(array_size: i64) -> Vec<Vec<f64>> {
    let mut input_array: Vec<Vec<f64>> = Vec::with_capacity(usize::from(array_size));
    for i in 0..array_size {
        for j in 0..array_size {
            // Pad the border with the correct value.
            if i == 0 || j == 0 || i == array_size - 1 || j == array_size - 1 {
                input_array[i][j] = function(i, j);
            } else {
                // All other values - values in the middle - should be 0.
                input_array[i][j] = 0.0;
            }
        }
    }

    return input_array;
}