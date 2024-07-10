use std::ops::{Index, IndexMut};

use crate::vectors::vect::Vect;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix {
    pub shape: (usize, usize),
    data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(shape: (usize, usize)) -> Self {
        let (rows, cols) = shape;
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![0.0; cols]);
        }
        Matrix {
            shape,
            data,
        }
    }

    pub fn from_vec(shape: (usize, usize), data: Vec<f32>) -> Self {
        let (rows, cols) = shape;
        let mut matrix = Matrix::new(shape);
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = data[i * cols + j];
            }
        }
        matrix
    }

    pub fn from_slice(shape: (usize, usize), data: &[f32]) -> Self {
        let (rows, cols) = shape;
        let mut matrix = Matrix::new(shape);
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = data[i * cols + j];
            }
        }
        matrix
    }

    pub fn from_slices(data: &[&[f32]]) -> Self {
        let rows = data.len();
        let cols = data.iter().map(|slice| slice.len()).min().unwrap();
        let mut matrix = Matrix::new((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = data[i][j];
            }
        }
        matrix
    }

    pub fn to_vecs(&self) -> Vec<Vec<f32>> {
        self.data.clone()
    }

    pub fn to_slices(&self) -> Vec<&[f32]> {
        self.data.iter().map(|row| row.as_slice()).collect()
    }

    pub fn to_vects(&self) -> Vec<Vect> {
        self.data.iter().map(|row| Vect::from_slice(row.as_slice())).collect()
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Matrix::new((size, size));
        for i in 0..size {
            matrix[(i, i)] = 1.0;
        }
        matrix
    }

    pub fn zeros(shape: (usize, usize)) -> Self {
        Matrix::new(shape)
    }

    pub fn ones(shape: (usize, usize)) -> Self {
        let (rows, cols) = shape;
        let mut matrix = Matrix::new(shape);
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = 1.0;
            }
        }
        matrix
    }

    pub fn from_vects(vects: Vec<Vect>) -> Self {
        let rows = vects.len();
        let cols = vects[0].dim;
        let mut matrix = Matrix::new((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = vects[i][j];
            }
        }
        matrix
    }

    pub fn transpose(&self) -> Self {
        let (rows, cols) = self.shape;
        let mut matrix = Matrix::new((cols, rows));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(j, i)] = self[(i, j)];
            }
        }
        matrix
    }

    pub fn dot(&self, other: &Matrix) -> Result<Matrix, MatrixError> {
        let (rows, cols) = self.shape;
        let (other_rows, other_cols) = other.shape;
        if cols != other_rows {
            return Err(MatrixError::DimensionMismatch(self.shape, other.shape));
        }

        let mut matrix = Matrix::new((rows, other_cols));
        for i in 0..rows {
            for j in 0..other_cols {
                let mut sum = 0.0;
                for k in 0..cols {
                    sum = sum + self[(i, k)] * other[(k, j)];
                }
                matrix[(i, j)] = sum;
            }
        }
        Ok(matrix)
    }

    pub fn row(&self, index: usize) -> Vec<f32> {
        self[index].clone()
    }

    pub fn column(&self, index: usize) -> Vec<f32> {
        let transposed = self.transpose();
        transposed[index].clone()
    }

    pub fn row_vect(&self, index: usize) -> Result<Vect, MatrixError> {
        if index > self.shape.1 {
            return Err(MatrixError::InvalidIndex((index, self.shape.1)));
        }
        let row = self[index].clone();
        Ok(Vect::from_vec(row))
    }

    pub fn column_vect(&self, index: usize) -> Result<Vect, MatrixError> {
        let transposed = self.transpose();
        transposed.row_vect(index)
    }

    pub fn rotation_matrix(axis: &Vect, angle: f32) -> Self {
        let (x, y, z) = (axis[0], axis[1], axis[2]);
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;

        let mut matrix = Matrix::identity(3);
        matrix[(0, 0)] = t * x * x + c;
        matrix[(0, 1)] = t * x * y - s * z;
        matrix[(0, 2)] = t * x * z + s * y;
        matrix[(1, 0)] = t * x * y + s * z;
        matrix[(1, 1)] = t * y * y + c;
        matrix[(1, 2)] = t * y * z - s * x;
        matrix[(2, 0)] = t * x * z - s * y;
        matrix[(2, 1)] = t * y * z + s * x;
        matrix[(2, 2)] = t * z * z + c;

        matrix
    }

    pub fn homogenous_translation_matrix(translation: &Vect) -> Self {
        let mut matrix = Matrix::identity(4);
        matrix[(0, 3)] = translation[0];
        matrix[(1, 3)] = translation[1];
        matrix[(2, 3)] = translation[2];
        matrix
    }

    pub fn homogenous_scale_matrix(scale: &Vect) -> Self {
        let mut matrix = Matrix::identity(4);
        matrix[(0, 0)] = scale[0];
        matrix[(1, 1)] = scale[1];
        matrix[(2, 2)] = scale[2];
        matrix
    }

    pub fn homogenous_rotation_matrix(axis: &Vect, angle: f32) -> Self {
        let (x, y, z) = (axis[0], axis[1], axis[2]);
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;

        let mut matrix = Matrix::identity(4);
        matrix[(0, 0)] = t * x * x + c;
        matrix[(0, 1)] = t * x * y - s * z;
        matrix[(0, 2)] = t * x * z + s * y;
        matrix[(1, 0)] = t * x * y + s * z;
        matrix[(1, 1)] = t * y * y + c;
        matrix[(1, 2)] = t * y * z - s * x;
        matrix[(2, 0)] = t * x * z - s * y;
        matrix[(2, 1)] = t * y * z + s * x;
        matrix[(2, 2)] = t * z * z + c;

        matrix
    }

    pub fn homogenous_slice() -> [[f32; 4]; 4] {
        let mut slice = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    slice[i][j] = 1.0;
                }
            }
        }
        slice
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        let (i, j) = index;
        &mut self.data[i][j]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vec<f32> {
        let row = &mut self.data[index];
        row
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        let (i, j) = index;
        &self.data[i][j]
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Vec<f32> {
        let row = &self.data[index];
        &row
    }
}

impl std::ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Matrix {
        let (rows, cols) = self.shape;
        let mut matrix = Matrix::new((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = -self[(i, j)];
            }
        }
        matrix
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Matrix {
        let (rows, cols) = self.shape;
        let mut matrix = Matrix::new((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        matrix
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Matrix {
        let (rows, cols) = self.shape;
        let mut matrix = Matrix::new((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = self[(i, j)] - rhs[(i, j)];
            }
        }
        matrix
    }
}

impl std::ops::Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f32) -> Matrix {
        let (rows, cols) = self.shape;
        let mut matrix = Matrix::new((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                matrix[(i, j)] = self[(i, j)] * rhs;
            }
        }
        matrix
    }
}

impl std::ops::Mul<Vect> for Matrix {
    type Output = Vect;

    fn mul(self, rhs: Vect) -> Vect {
        let (rows, cols) = self.shape;

        let mut result = Vec::new();
        for i in 0..rows {
            let mut sum = 0.0;
            for j in 0..cols {
                sum = sum + self[(i, j)] * rhs[j];
            }
            result.push(sum);
        }

        Vect::from_slice(&result)
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        self.dot(&rhs).unwrap()
    }
}

#[derive(Debug)]
pub enum MatrixError {
    DimensionMismatch((usize, usize), (usize, usize)),
    InvalidIndex((usize, usize)),
}

impl std::fmt::Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MatrixError::DimensionMismatch((rows, cols), (other_rows, other_cols)) => {
                write!(f, "Matrix dimension mismatch: ({}, {}) != ({}, {})", rows, cols, other_rows, other_cols)
            }
            MatrixError::InvalidIndex((index, max)) => {
                write!(f, "Invalid index: {} > {}", index, max)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let matrix_size = 4;
        let mat = Matrix::identity(matrix_size);

        for i in 0..matrix_size {
            for j in 0..matrix_size {
                if i == j {
                    assert_eq!(mat[(i, j)], 1.0);
                } else {
                    assert_eq!(mat[(i, j)], 0.0);
                }
            }
        }
    }

    #[test]
    fn test_transpose() {
        let shape = (3, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ];
        let mat = Matrix::from_vec(shape, data);
        let transposed = mat.transpose();

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                assert_eq!(mat[(i, j)], transposed[(j, i)]);
            }
        }
    }

    #[test]
    fn test_dot() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let other_shape = (3, 2);
        let other_data = vec![
            7.0, 8.0,
            9.0, 10.0,
            11.0, 12.0,
        ];
        let other_mat = Matrix::from_vec(other_shape, other_data);

        let result = mat.dot(&other_mat).unwrap();
        let expected_shape = (2, 2);
        let expected_data = vec![
            58.0, 64.0,
            139.0, 154.0,
        ];
        let expected = Matrix::from_vec(expected_shape, expected_data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_row() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let row = mat.row(1);
        assert_eq!(row, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_column() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let column = mat.column(1);
        assert_eq!(column, vec![2.0, 5.0]);
    }

    #[test]
    fn test_row_vect() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let row = mat.row_vect(1).unwrap();
        assert_eq!(row.dim, 3);
        assert_eq!(row[0], 4.0);
        assert_eq!(row[1], 5.0);
        assert_eq!(row[2], 6.0);
    }

    #[test]
    fn test_column_vect() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let column = mat.column_vect(1).unwrap();
        assert_eq!(column.dim, 2);
        assert_eq!(column[0], 2.0);
        assert_eq!(column[1], 5.0);
    }

    #[test]
    fn test_neg() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let neg = -mat;
        let expected_data = vec![
            -1.0, -2.0, -3.0,
            -4.0, -5.0, -6.0,
        ];
        let expected = Matrix::from_vec(shape, expected_data);

        assert_eq!(neg, expected);
    }

    #[test]
    fn test_add() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let other_data = vec![
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0,
        ];
        let other = Matrix::from_vec(shape, other_data);

        let sum = mat + other;
        let expected_data = vec![
            8.0, 10.0, 12.0,
            14.0, 16.0, 18.0,
        ];
        let expected = Matrix::from_vec(shape, expected_data);

        assert_eq!(sum, expected);
    }

    #[test]
    fn test_sub() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let other_data = vec![
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0,
        ];
        let other = Matrix::from_vec(shape, other_data);

        let diff = mat - other;
        let expected_data = vec![
            -6.0, -6.0, -6.0,
            -6.0, -6.0, -6.0,
        ];
        let expected = Matrix::from_vec(shape, expected_data);

        assert_eq!(diff, expected);
    }

    #[test]
    fn test_mul_scalar() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let scalar = 2.0;
        let product = mat * scalar;
        let expected_data = vec![
            2.0, 4.0, 6.0,
            8.0, 10.0, 12.0,
        ];
        let expected = Matrix::from_vec(shape, expected_data);

        assert_eq!(product, expected);
    }

    #[test]
    fn test_mul_vector() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let vect = Vect::from_vec(vec![1.0, 2.0, 3.0]);
        let product = mat * vect;
        let expected = Vect::from_vec(vec![14.0, 32.0]);

        assert_eq!(product, expected);
    }

    #[test]
    fn test_mul_matrix() {
        let shape = (2, 3);
        let data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let mat = Matrix::from_vec(shape, data);

        let other_shape = (3, 2);
        let other_data = vec![
            7.0, 8.0,
            9.0, 10.0,
            11.0, 12.0,
        ];
        let other = Matrix::from_vec(other_shape, other_data);

        let product = mat * other;
        let expected_shape = (2, 2);
        let expected_data = vec![
            58.0, 64.0,
            139.0, 154.0,
        ];
        let expected = Matrix::from_vec(expected_shape, expected_data);

        assert_eq!(product, expected);
    }

    #[test]
    fn test_from_vects() {
        let vects = vec![
            Vect::from_vec(vec![1.0, 2.0, 3.0]),
            Vect::from_vec(vec![4.0, 5.0, 6.0]),
        ];
        let mat = Matrix::from_vects(vects);

        let expected_shape = (2, 3);
        let expected_data = vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ];
        let expected = Matrix::from_vec(expected_shape, expected_data);

        assert_eq!(mat, expected);
    }

    #[test]
    fn test_zeros() {
        let shape = (2, 3);
        let mat = Matrix::zeros(shape);

        let expected_data = vec![
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        let expected = Matrix::from_vec(shape, expected_data);

        assert_eq!(mat, expected);
    }
}