#[allow(dead_code)]
use std::ops;
use std::fmt;

#[derive(Clone)]
pub struct Array2D<T> {
    data: Vec<Vec<T>>,
    dim: (usize, usize)
}

impl<T> Array2D<T> where T: Default + Clone {
    pub fn zeros(dim: (usize, usize)) -> Array2D<T> {
        let (n, m) = dim;
        let mut data = vec![Vec::new(); m];
        for item in data.iter_mut() {
            *item = vec![T::default(); n];
        }
        Array2D { data: data, dim: dim }
    }

    pub fn from(v: &[&[T]]) -> Array2D<T> {
        let dim = (v.len(), v[0].len());
        let mut data = vec![Vec::new(); dim.0];
        for (m, e) in data.iter_mut().zip(v) {
            *m = Vec::from(*e);
        }
        Array2D { data: data, dim: dim }
    }
}

impl<T> Array2D<T> {
    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }
}

impl<T> ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &(self.data[index.0])[index.1]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut (self.data[index.0])[index.1]
    }
}

impl<T> fmt::Debug for Array2D<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}