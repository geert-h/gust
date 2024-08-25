use std::ops::{Index, IndexMut};

use crate::vectors::vect4::Vect4;

#[derive(Copy)]
pub struct Vect3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vect3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vect3 {
        Vect3 {
            x,
            y,
            z,
        }
    }

    pub fn from_slice(data: &[f32]) -> Vect3 {
        Vect3 {
            x: data[0],
            y: data[1],
            z: data[2],
        }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) -> Vect3 {
        let norm = self.norm();
        if norm == 0.0 {
            return self.clone();
        }
        self.x = self.x / norm;
        self.y = self.y / norm;
        self.z = self.z / norm;
        self.clone()
    }

    pub fn cross(&self, other: &Vect3) -> Vect3 {
        Vect3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vect3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn as_slice(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }

    pub fn to_vect4(&self, value: f32) -> Vect4 {
        Vect4::from_slice(&[self.x, self.y, self.z, value])
    }
}

impl Clone for Vect3 {
    fn clone(&self) -> Vect3 {
        Vect3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl std::ops::Add<Vect3> for Vect3 {
    type Output = Vect3;

    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Index<usize> for Vect3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Vect3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl std::ops::Sub<Vect3> for Vect3 {
    type Output = Vect3;

    fn sub(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vect3 {
    type Output = Vect3;

    fn mul(self, rhs: f32) -> Vect3 {
        Vect3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vect3> for f32 {
    type Output = Vect3;

    fn mul(self, rhs: Vect3) -> Vect3 {
        Vect3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Neg for Vect3 {
    type Output = Vect3;

    fn neg(self) -> Vect3 {
        Vect3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::fmt::Display for Vect3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::fmt::Debug for Vect3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vect3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Vect3 {
    fn eq(&self, other: &Vect3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}