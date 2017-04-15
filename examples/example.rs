extern crate arrays;

use arrays::Array2D;

fn main() {
    let mut m = Array2D::from(&[
        &[1, 2, 3],
        &[4, 5, 6],
        &[0, 0, 0]
    ]);
    println!("M = {:?}\n", m);
    // IndexMut & Index
    for i in 0..3 {
        m[(2, i)] = m[(0, i)] + m[(1, i)];
    }
    // Iterate over &T (IntoIter with borrowing)
    for line in &m {
        println!("{:?}", line);
    }
    println!();
    // IterMut (by custom function)
    for line in m.iter_mut() {
        for item in line.iter_mut() {
            *item *= 2;
        }
    }
    // Iterate over T (default IntoIter)
    for line in m {
        println!("{:?}", line);
    }
}