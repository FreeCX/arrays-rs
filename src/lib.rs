//! Simple library for working with N-dimensional arrays (where N=2)
#![crate_name="arrays"]
#![warn(missing_docs)]

/// Two dimensional array module
pub mod array2d;
// re-export
pub use array2d::Array2D;