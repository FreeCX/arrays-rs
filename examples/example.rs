extern crate arrays;

use arrays::Array2D;

fn main() {
    // Vec array
    let vec_data = vec![
        vec![1, 2, 3, 4, 5, 6],
        vec![4, 5, 6, 7, 8, 9],
        vec![7, 8, 9, 1, 2, 3]
    ];
    // slice array
    let slice_data: &[&[u8]] = &[
        &[1, 2, 3],
        &[4, 5, 6],
        &[0, 0, 0]
    ];
    // you can take data from Vec
    let s_array = Array2D::from(vec_data);
    // or slice array
    let mut m = Array2D::from(slice_data);
    println!("s_array = {:?}", s_array);
    println!("M = {:?}", m);
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