use crate::vectors::vect3::Vect3;
use crate::vectors::vect4::Vect4;

pub struct Mat4 {
    data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new() -> Self {
        Mat4 {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn from_vec(data: Vec<f32>) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = data[i * 4 + j];
            }
        }
        mat
    }

    pub fn from_slice(data: &[f32]) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = data[i * 4 + j];
            }
        }
        mat
    }

    pub fn from_slices(slices: [[f32; 3]; 4]) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..3 {
                mat.data[i][j] = slices[i][j];
            }
        }
        mat
    }

    pub fn from_vects(vects: [Vect4; 4]) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = vects[j][i];
            }
        }
        mat
    }

    pub fn identity() -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            mat.data[i][i] = 1.0;
        }
        mat
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut vec = Vec::with_capacity(16);
        for i in 0..4 {
            for j in 0..4 {
                vec.push(self.data[i][j]);
            }
        }
        vec
    }

    pub fn to_slice(&self) -> [f32; 16] {
        let mut slice = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                slice[i * 4 + j] = self.data[i][j];
            }
        }
        slice
    }

    pub fn to_slices(&self) -> [[f32; 4]; 4] {
        self.data
    }

    pub fn transpose(&self) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = self.data[j][i];
            }
        }
        mat
    }

    pub fn scale(&self, scale: Vect3) -> Self {
        let mut mat = Mat4::identity();
        mat.data[0][0] = scale[0];
        mat.data[1][1] = scale[1];
        mat.data[2][2] = scale[2];
        mat
    }

    pub fn rotate(&self, angle: f32, axis: Vect3) -> Self {
        let mut mat = Mat4::identity();
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        mat.data[0][0] = t * x * x + c;
        mat.data[0][1] = t * x * y - s * z;
        mat.data[0][2] = t * x * z + s * y;
        mat.data[1][0] = t * x * y + s * z;
        mat.data[1][1] = t * y * y + c;
        mat.data[1][2] = t * y * z - s * x;
        mat.data[2][0] = t * x * z - s * y;
        mat.data[2][1] = t * y * z + s * x;
        mat.data[2][2] = t * z * z + c;
        mat
    }

    pub fn rotate_with_dir_and_up(&self, direction: Vect3, up: Vect3) -> Self {
        let mut mat = Mat4::identity();
        let right = direction.cross(&up).normalize();
        let new_up = right.cross(&direction).normalize();
        mat.data[0][0] = right.x;
        mat.data[0][1] = right.y;
        mat.data[0][2] = right.z;
        mat.data[1][0] = new_up.x;
        mat.data[1][1] = new_up.y;
        mat.data[1][2] = new_up.z;
        mat.data[2][0] = direction.x;
        mat.data[2][1] = direction.y;
        mat.data[2][2] = direction.z;
        mat
    }

    pub fn dot(&self, other: &Mat4) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j]
                    + self.data[i][3] * other.data[3][j];
            }
        }
        mat
    }

    pub fn translate(&self, translation: Vect3) -> Self {
        let mut mat = Mat4::identity();
        mat.data[3][0] = translation[0];
        mat.data[3][1] = translation[1];
        mat.data[3][2] = translation[2];
        self.dot(&mat)
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.dot(&other)
    }
}

impl std::ops::Mul<Vect3> for Mat4 {
    type Output = Vect3;

    fn mul(self, other: Vect3) -> Vect3 {
        let mut vec = Vect3::new(0.0, 0.0, 0.0);
        for i in 0..3 {
            vec[i] = self.data[i][0] * other[0]
                + self.data[i][1] * other[1]
                + self.data[i][2] * other[2]
                + self.data[i][3];
        }
        vec
    }
}

impl std::ops::Mul<Vect4> for Mat4 {
    type Output = Vect4;

    fn mul(self, other: Vect4) -> Vect4 {
        let mut vec = Vect4::new();
        for i in 0..4 {
            vec[i] = self.data[i][0] * other[0]
                + self.data[i][1] * other[1]
                + self.data[i][2] * other[2]
                + self.data[i][3] * other[3];
        }
        vec
    }
}

impl std::ops::Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = self.data[i][j] * other;
            }
        }
        mat
    }
}

impl std::ops::Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = self * other.data[i][j];
            }
        }
        mat
    }
}

impl std::ops::Add for Mat4 {
    type Output = Mat4;

    fn add(self, other: Mat4) -> Mat4 {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        mat
    }
}

impl std::ops::Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, other: Mat4) -> Mat4 {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        mat
    }
}

impl std::ops::Neg for Mat4 {
    type Output = Mat4;

    fn neg(self) -> Mat4 {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.data[i][j] = -self.data[i][j];
            }
        }
        mat
    }
}

impl std::ops::Index<usize> for Mat4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}