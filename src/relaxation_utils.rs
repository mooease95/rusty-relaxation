

pub fn check_precision(correct_array: &Vec<Vec<f64>>, relaxed_value:f64, row: usize, column: usize, target_precision: f64) -> bool {
    let correct_value: f64 = correct_array[row][column];
    // WARNING: The below will lose some precision!
    let precision_reached: f64 = (correct_value - relaxed_value).abs();
    precision_reached <= target_precision
}

pub fn average_array(array: &Vec<Vec<f64>>, x: usize, y: usize) -> f64 {
    let value: f64 = array[x][y-1] + array[x][y+1] + array[x-1][y] + array[x+1][y];
    value / 4.0
}