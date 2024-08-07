
// Assume equation is f(x,y) = x + y
pub fn initialise_array(array_size: i64) -> Vec<Vec<i64>> {
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