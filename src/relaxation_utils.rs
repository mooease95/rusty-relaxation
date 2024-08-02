
/*
    public static double averageArray(double[][] array, int x, int y) {
        return (array[x][y-1] + array[x][y+1] + array[x-1][y] + array[x+1][y])/4;
    }
 */

fn average_array(array: [[i32; 4]; 4], x: i32, y: i32) -> i32 {
    let value = (array[x][y-1] + array[x][y+1] + array[x-1][y] + array[x+1][y])/4; // TODO: <=== What the hell is this issue? Work here.
}