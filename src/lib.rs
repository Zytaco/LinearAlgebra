use std::{iter::Map, collections::btree_map::Range};


#[derive(Clone, Copy, Debug)]
struct Matrix<T, const N: usize, const M: usize> (
    [[T; N]; M]
);

impl<T, const N: usize, const M:usize> From<[[T; N]; M]> for Matrix<T, N, M> {
    fn from(array2d: [[T; N]; M]) -> Self {
        Matrix (array2d)
    }
}

type Vector<T, const N:usize> = Matrix<T, N, 1>;
impl<T> From<T> for Vector<T, 1> {
    fn from(t: T) -> Self {
        Matrix ([[t]])
    }
}
impl<T> From<(T, T)> for Vector<T, 2> {
    fn from(tuple: (T, T)) -> Self {
        Matrix ([[tuple.0, tuple.1]])
    }
}
impl<T> From<(T, T, T)> for Vector<T, 3> {
    fn from(tuple: (T, T, T)) -> Self {
        Matrix ([[tuple.0, tuple.1, tuple.2]])
    }
}


#[cfg(test)]
mod tests {
    extern crate std;
    use std::{
        assert_eq,
        convert::{
            Into
        },
        format,
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn debug_is_same() {
        let m=  [[1i32, 2, 3], [4, 5, 6]];
        use crate::Matrix;
        let matrix: Matrix<i32, 3, 2> = m.into();
        assert_eq!(format!("Matrix({:?})", m), format!("{:?}", matrix));
    }
}
