extern crate arrays;

use std::u8;
use arrays::Array2D;

// https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm
fn floyd_warshall(w: &mut Array2D<u8>) -> Array2D<u8> {
    let n = w.dim().0;
    let mut next = Array2D::zeros((n, n));
    for v in 0..n {
        for u in 0..n {
            if (w[(v, u)] != u8::max_value()) & (v != u) {
                next[(v, u)] = v as u8;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            if w[(i, k)] == u8::max_value() {
                continue;
            }
            for j in 0..n {
                let (res, oflag) = w[(i, k)].overflowing_add(w[(k, j)]);
                if !oflag & (w[(i, j)] > res) {
                    w[(i, j)] = res;
                    next[(i, j)] = next[(k, j)];
                }
            }
        }
    }
    next
}

fn restore_path((u, v): (usize, usize), pred: &Array2D<u8>) -> Vec<usize> {
    if pred[(u, v)] as usize == u {
        vec![u, v]
    } else {
        let mut left = restore_path((u, pred[(u, v)] as usize), pred);
        let right = restore_path((pred[(u, v)] as usize, v), pred);
        left.extend_from_slice(&right[1..]);
        left
    }
}

fn print_sep(t: &str, s: char, len: usize) {
    let separator: String = (0..len).map(|_| s).collect();
    println!("{}\n {}\n{}", separator, t, separator);
}

fn main() {
    let i = u8::max_value();
    let data: &[&[_]] = &[
        &[0, 1, i, i, i, i, 1, i], 
        &[1, 0, 1, i, i, i, 10, i], 
        &[i, 1, 0, 1, i, i, i, i], 
        &[i, i, 1, 0, i, i, i, i], 
        &[i, i, i, i, 0, 1, 1, i], 
        &[i, i, i, i, 1, 0, i, 1], 
        &[1, 10, i, i, 1, i, 0, i], 
        &[i, i, i, i, i, 1, i, 0]
    ];
    let mut m = Array2D::from(data);
    print_sep("Matrix", '-', 50);
    for line in &m {
        for item in line {
            print!("{:4} ", item);
        }
        println!();
    }
    let r = floyd_warshall(&mut m);
    print_sep("Result (matrix | predecessors)", '-', 50);
    for (r_line, m_line) in r.iter().zip(&m) {
        for item in m_line {
            print!("{:2} ", item);
        }
        print!(" |");
        for item in r_line {
            print!("{:2} ", item);
        }
        println!();
    }
    print_sep("Paths", '-', 50);
    let n = r.dim().0;
    for i in 0..n {
        for j in i..n {
            if i == j {
                continue;
            }
            println!("<{}..{}> = {:?}", i, j, restore_path((i, j), &r));
        }
    }
}
