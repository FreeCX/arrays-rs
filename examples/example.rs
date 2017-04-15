extern crate arrays;

use arrays::Array2D;

fn main() {
    let mut m = Array2D::from(&[
        &[1, 2, 3],
        &[4, 5, 6],
        &[0, 0, 0]
    ]);
    println!("M = {:?}", m);
    for i in 0..3 {
        m[(2, i)] = m[(0, i)] + m[(1, i)];
    }
    println!("M = {:?}", m);
}