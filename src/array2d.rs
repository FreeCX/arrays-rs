#[allow(dead_code)]
use std::ops;
use std::fmt;
use std::vec;
use std::slice;

#[derive(Clone)]
/// Two dimensional array
pub struct Array2D<T> {
    data: Vec<Vec<T>>,
    dim: (usize, usize)
}

impl<T> Array2D<T> where T: Default + Clone {
    /// Initialize dim-sized array of zeros (default value)
    ///
    /// # Example
    /// ```
    /// use arrays::Array2D;
    ///
    /// let a: Array2D<u8> = Array2D::zeros((3, 4));
    /// println!("a = {:?}", a);
    /// // a = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]]
    /// ```
    pub fn zeros(dim: (usize, usize)) -> Array2D<T> {
        let (n, m) = dim;
        let mut data = vec![Vec::new(); m];
        for item in data.iter_mut() {
            *item = vec![T::default(); n];
        }
        Array2D { data: data, dim: dim }
    }
}

impl<T> Array2D<T> {
    /// Size of Array2D (row, column)
    /// 
    /// # Example
    /// ```
    /// use arrays::Array2D;
    ///
    /// let m: Array2D<u8> = Array2D::zeros((3, 4));
    /// assert_eq!(m.dim(), (3, 4));
    /// ```
    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    /// Performs the conversion from Array2D to Vec
    ///
    /// # Example
    /// ```
    /// use arrays::Array2D;
    ///
    /// let slice_obj: &[&[_]] = &[
    ///     &[1, 2, 3],
    ///     &[4, 5, 6]
    /// ];
    /// let vec_obj = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6]
    /// ];
    /// let m = Array2D::from(slice_obj);
    /// let res = m.to_vec();
    /// assert_eq!(res, vec_obj);
    /// ```
    pub fn to_vec(self) -> Vec<Vec<T>> {
        self.data
    }
}

impl<T> From<Vec<Vec<T>>> for Array2D<T> {
    /// Performs the conversion from Vec to Array2D
    ///
    /// # Example
    /// ```
    /// use arrays::Array2D;
    ///
    /// let vec_obj = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9]
    /// ];
    /// let m = Array2D::from(vec_obj);
    /// let r = format!("m = {:?}", m);
    /// assert_eq!(r, "m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]");
    /// ```
    fn from(v: Vec<Vec<T>>) -> Self {
        let dim = (v.len(), v[0].len());
        Array2D { data: v, dim: dim }
    }
}

impl<'a, T: Clone> From<&'a [&'a [T]]> for Array2D<T> {
    /// Performs the conversion from slice to Array2D
    ///
    /// # Example
    /// ```
    /// use arrays::Array2D;
    ///
    /// let slice_obj: &[&[_]] = &[
    ///     &[1, 2, 3],
    ///     &[4, 5, 6]
    /// ];
    /// let m = Array2D::from(slice_obj);
    /// let r = format!("m = {:?}", m);
    /// assert_eq!(r, "m = [[1, 2, 3], [4, 5, 6]]");
    /// ```
    fn from(v: &'a [&'a [T]]) -> Self {
        let dim = (v.len(), v[0].len());
        let mut data = vec![Vec::new(); dim.0];
        for (m, e) in data.iter_mut().zip(v) {
            *m = Vec::from(*e);
        }
        Array2D { data: data, dim: dim }
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

impl<T> IntoIterator for Array2D<T> {
    type Item = Vec<T>;
    type IntoIter = vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Array2D<T> {
    type Item = &'a Vec<T>;
    type IntoIter = slice::Iter<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

// TODO: rewrite it
impl<'a, T> Array2D<T> {
    /// Returns an iterator over the slice.
    pub fn iter(&'a self) -> slice::Iter<'a, Vec<T>> {
        self.data.iter()
    }

    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&'a mut self) -> slice::IterMut<'a, Vec<T>> {
        self.data.iter_mut()
    }
}