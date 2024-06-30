use std::ops::{Index, IndexMut};
use crate::vect::Vect;

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

    pub fn row(&self, index: usize) -> Vect {
        self[index].clone()
    }

    pub fn column(&self, index: usize) -> Vect {
        let transposed = self.transpose();
        transposed[index].clone()
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        let (i, j) = index;
        &mut self.data[i][j]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vect {
        let row = &mut self.data[index];
        let vect = Vect::from_slice(row);
        &vect
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
    type Output = Vect;

    fn index(&self, index :usize) -> &Vect {
        let row = &self.data[index];
        let vect = Vect::from_slice(row);
        &vect
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

pub enum MatrixError {
    DimensionMismatch((usize, usize), (usize, usize)),
    InvalidIndex((usize, usize)),
}