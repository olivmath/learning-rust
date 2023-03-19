use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Matrix3x3 {
    pub data: [[i32; 3]; 3],
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Matrices {
    pub a: Matrix3x3,
    pub b: Matrix3x3,
}

impl Matrices {
    pub fn new(a: Matrix3x3, b: Matrix3x3) -> Matrices {
        Matrices { a, b }
    }
}

impl Matrix3x3 {
    pub fn new(data: [[i32; 3]; 3]) -> Matrix3x3 {
        Matrix3x3 { data }
    }

    pub fn transpose(self) -> Matrix3x3 {
        let mut data = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                data[i][j] = self.data[j][i];
            }
        }
        return Matrix3x3 { data };
    }

    pub fn mul(self, other: Matrix3x3) -> Matrix3x3 {
        let mut result = [[0; 3]; 3];
        for row in 0..3 {
            for column in 0..3 {
                for i in 0..3 {
                    result[row][column] += self.data[row][i] * other.data[i][column];
                }
            }
        }
        Matrix3x3::new(result)
    }
}

impl fmt::Display for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[\n");
        for i in &self.data {
            write!(f, "    [ ");
            for j in i {
                write!(f, "{} ", j);
            }
            write!(f, "]\n");
        }
        write!(f, "]\n")
    }
}

mod test {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = Matrix3x3::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let matrix_t = Matrix3x3::new([[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
        assert_eq!(matrix.transpose(), matrix_t)
    }

    #[test]
    fn test_matrices() {
        let a = Matrix3x3::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let b = Matrix3x3::new([[2, 2, 2], [2, 2, 2], [2, 2, 2]]);
        let matrices = Matrices::new(a, b);
        let result = Matrix3x3::new([[12, 12, 12], [30, 30, 30], [48, 48, 48]]);
        assert_eq!(matrices.a.mul(matrices.b.transpose()), result);
    }
}
