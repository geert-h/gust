use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vect {
    pub dim: usize,
    data: Vec<f32>,
}

impl Vect {
    pub fn new(dim: usize) -> Self {
        let mut data = Vec::with_capacity(dim);
        for _ in 0..dim {
            data.push(0.0);
        }

        Vect {
            dim,
            data,
        }
    }

    pub fn from_vec(data: Vec<f32>) -> Self {
        let dim = data.len();
        Vect {
            dim,
            data,
        }
    }

    pub fn from_slice(data: &[f32]) -> Self {
        let dim = data.len();
        let mut vec = Vec::with_capacity(dim);
        for i in 0..dim {
            vec.push(data[i]);
        }
        Vect {
            dim,
            data: vec,
        }
    }

    pub fn norm(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.dim {
            sum = sum + self[i] * self[i];
        }

        let root = sum.sqrt();

        if root.is_nan() {
            return 0.0;
        }
        root
    }

    pub fn normalize(&mut self) -> Vect {
        let norm = self.norm();
        if norm == 0.0 {
            return self.clone();
        }
        for i in 0..self.dim {
            self.data[i] = self.data[i] / norm;
        }
        self.clone()
    }

    pub fn dot(&self, other: &Vect) -> Result<f32, VectError> {
        if self.dim != other.dim {
            return Err(VectError::DimensionMismatch(self.dim, other.dim));
        }

        let mut sum = 0.0;
        for i in 0..self.dim {
            sum = sum + self[i] * other[i];
        }
        Ok(sum)
    }

    pub fn cross(&self, other: &Vect) -> Result<Vect, VectError> {
        if (self.dim < 2 || self.dim > 3) && (other.dim < 2 || other.dim > 3) {
            return Err(VectError::InvalidCrossDimension);
        }

        if self.dim != other.dim {
            return Err(VectError::DimensionMismatch(self.dim, other.dim));
        }

        let mut result = Vect::new(self.dim);

        if self.dim == 2 {
            result[0] = self[0] * other[1] - self[1] * other[0];
        } else {
            result[0] = self[1] * other[2] - self[2] * other[1];
            result[1] = self[2] * other[0] - self[0] * other[2];
            result[2] = self[0] * other[1] - self[1] * other[0];
        }

        Ok(result)
    }

    pub fn set(&mut self, index: usize, value: f32) {
        if index >= self.dim {
            panic!("Invalid index");
        }

        self.data[index] = value;
    }

    pub fn to_vec(&self) -> Vec<f32> {
        self.data.clone()
    }
}

impl std::ops::Mul<f32> for Vect {
    type Output = Vect;

    fn mul(self, rhs: f32) -> Vect {
        let mut result = Vect::new(self.dim);
        for i in 0..self.dim {
            result[i] = self[i] * rhs;
        }
        result
    }
}

impl std::ops::Add for Vect {
    type Output = Vect;

    fn add(self, rhs: Vect) -> Vect {
        let mut result = Vect::new(self.dim);
        for i in 0..self.dim {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}

impl std::ops::Sub for Vect {
    type Output = Vect;

    fn sub(self, rhs: Vect) -> Vect {
        let mut result = Vect::new(self.dim);
        for i in 0..self.dim {
            result[i] = self[i] - rhs[i];
        }
        result
    }
}

impl std::ops::Neg for Vect {
    type Output = Vect;

    fn neg(self) -> Vect {
        let mut result = Vect::new(self.dim);
        for i in 0..self.dim {
            result[i] = -self[i];
        }
        result
    }
}

impl Index<usize> for Vect {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        if index >= self.dim {
            panic!("Invalid index");
        }

        &self.data[index]
    }
}

impl IndexMut<usize> for Vect {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        if index >= self.dim {
            panic!("Invalid index");
        }

        &mut self.data[index]
    }
}

//Define errors
#[derive(Debug)]
pub enum VectError {
    DimensionMismatch(usize, usize),
    InvalidCrossDimension,
    InvalidIndex,
}

impl std::fmt::Display for VectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VectError::InvalidCrossDimension => write!(f, "Cross product is only defined for 2D and 3D vectors"),
            VectError::DimensionMismatch(self_dim, other_dim) => write!(f, "Dimension mismatch: {} != {}", self_dim, other_dim),
            VectError::InvalidIndex => write!(f, "Invalid index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        let mut v = Vect::from_slice(&[1.0, 2.0, 3.0]);

        assert_eq!(v.norm(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        let mut v = Vect::from_slice(&[1.0, 2.0, 0.0]);

        v.normalize();
        let eps = 1e-6;

        // Check if the norm is 1 +- eps
        assert!((v.norm() - 1.0).abs() < eps);
    }

    #[test]
    fn test_dot() {
        let mut v1 = Vect::from_slice(&[1.0, 2.0, 3.0]);

        let v2 = Vect::from_slice(&[4.0, 5.0, 6.0]);

        assert_eq!(v1.dot(&v2).unwrap(), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vect::from_slice(&[1.0, 2.0, 3.0]);

        let v2 = Vect::from_slice(&[4.0, 5.0, 6.0]);

        let result = v1.cross(&v2).unwrap();

        assert_eq!(result[0], -3.0);
        assert_eq!(result[1], 6.0);
        assert_eq!(result[2], -3.0);
    }

    #[test]
    fn test_cross_invalid_dimension() {
        let v1 = Vect::from_slice(&[1.0, 2.0]);

        let v2 = Vect::from_slice(&[4.0, 5.0, 6.0]);

        let result = v1.cross(&v2);

        assert!(result.is_err());
    }

    #[test]
    fn test_cross_dimension_valid() {
        let v1 = Vect::from_slice(&[1.0, 2.0, 3.0]);

        let v2 = Vect::from_slice(&[4.0, 5.0, 6.0]);

        let result = v1.cross(&v2);

        assert!(result.is_ok());
    }
}