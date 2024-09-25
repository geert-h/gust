use std::ops::{Index, IndexMut};

use crate::vectors::vect3::Vect3;

#[derive(Copy, Debug)]
pub struct Mat3 {
    pub data: [[f32; 3]; 3],
}

impl Mat3 {
    pub fn new() -> Mat3 {
        Mat3 {
            data: [[0.0; 3]; 3],
        }
    }

    pub fn identity() -> Mat3 {
        let mut mat = Mat3::new();
        for i in 0..3 {
            mat.data[i][i] = 1.0;
        }
        mat
    }

    pub fn from_slice(slice: &[[f32; 3]; 3]) -> Mat3 {
        Mat3 {
            data: slice.clone(),
        }
    }

    pub fn to_array(&self) -> [[f32; 3]; 3] {
        self.data.clone()
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                vec.push(self.data[i][j]);
            }
        }
        vec
    }

    pub fn cross(&self, other: &Mat3) -> Mat3 {
        let mut res = Mat3::new();
        for i in 0..3 {
            for j in 0..3 {
                res.data[i][j] = self.data[i][0] * other.data[0][j] + self.data[i][1] * other.data[1][j] + self.data[i][2] * other.data[2][j];
            }
        }
        res
    }

    pub fn transpose(&self) -> Mat3 {
        let mut res = Mat3::new();
        for i in 0..3 {
            for j in 0..3 {
                res.data[i][j] = self.data[j][i];
            }
        }
        res
    }

    pub fn det(&self) -> f32 {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1]) - self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0]) + self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }

    pub fn inverse(&self) -> Mat3 {
        let det = self.det();
        if det == 0.0 {
            panic!("Matrix is not invertible");
        }
        let mut res = Mat3::new();
        let inv_det = 1.0 / det;
        res.data[0][0] = (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1]) * inv_det;
        res.data[0][1] = (self.data[0][2] * self.data[2][1] - self.data[0][1] * self.data[2][2]) * inv_det;
        res.data[0][2] = (self.data[0][1] * self.data[1][2] - self.data[0][2] * self.data[1][1]) * inv_det;
        res.data[1][0] = (self.data[1][2] * self.data[2][0] - self.data[1][0] * self.data[2][2]) * inv_det;
        res.data[1][1] = (self.data[0][0] * self.data[2][2] - self.data[0][2] * self.data[2][0]) * inv_det;
        res.data[1][2] = (self.data[1][0] * self.data[0][2] - self.data[0][0] * self.data[1][2]) * inv_det;
        res.data[2][0] = (self.data[1][0] * self.data[2][1] - self.data[2][0] * self.data[1][1]) * inv_det;
        res.data[2][1] = (self.data[2][0] * self.data[0][1] - self.data[0][0] * self.data[2][1]) * inv_det;
        res.data[2][2] = (self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]) * inv_det;
        res
    }

    pub fn dot(&self, other: &Mat3) -> f32 {
        let mut res = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                res += self.data[i][j] * other.data[i][j];
            }
        }
        res
    }

    pub fn rotation_matrix(axis: &Vect3, angle: f32) -> Mat3 {
        let mut res = Mat3::new();
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        let x = axis[0];
        let y = axis[1];
        let z = axis[2];
        res.data[0][0] = t * x * x + c;
        res.data[0][1] = t * x * y - s * z;
        res.data[0][2] = t * x * z + s * y;
        res.data[1][0] = t * x * y + s * z;
        res.data[1][1] = t * y * y + c;
        res.data[1][2] = t * y * z - s * x;
        res.data[2][0] = t * x * z - s * y;
        res.data[2][1] = t * y * z + s * x;
        res.data[2][2] = t * z * z + c;
        res
    }
}

impl Clone for Mat3 {
    fn clone(&self) -> Mat3 {
        Mat3 {
            data: self.data.clone(),
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, other: Mat3) -> Mat3 {
        self.cross(&other)
    }
}

impl std::ops::Mul<Vect3> for Mat3 {
    type Output = Vect3;

    fn mul(self, other: Vect3) -> Vect3 {
        let mut res = Vect3::new(0.0, 0.0, 0.0);
        for i in 0..3 {
            for j in 0..3 {
                res[i] += self.data[i][j] * other[j];
            }
        }
        res
    }
}