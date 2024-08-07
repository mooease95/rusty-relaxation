

pub fn check_precision(correct_array: [[f64; 4]; 4], relaxed_value:f64, row: usize, column: usize, target_precision: f64) -> bool {
    let correct_value: f64 = correct_array[row][column];
    let precision_reached: f64 = (correct_value - relaxed_value).abs();
    return precision_reached <= target_precision;
}

pub fn average_array(array: [[f64; 4]; 4], x: usize, y: usize) -> f64 {
    let value: f64 = array[x][y-1] + array[x][y+1] + array[x-1][y] + array[x+1][y];
    return value / 4.0;
}