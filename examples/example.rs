extern crate matrix;

use matrix::Matrix;

fn main() {
    let mut m = Matrix::from(&[
        &[1, 2, 3],
        &[4, 5, 6],
        &[7, 8, 9]
    ]);
    println!("M = {:?}", m);
    m[(1, 1)] = 10;
    println!("M[1, 1] = {}", m[(1, 1)]);
    println!("M = {:?}", m);
}