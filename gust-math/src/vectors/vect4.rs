use crate::vectors::vect::Vect;

pub struct Vect4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vect4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vect4 {
        Vect4 { x, y, z, w }
    }

    pub fn from_slice(slice: &[f32; 4]) -> Vect4 {
        Vect4 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z, self.w]
    }

    pub fn to_vect(&self) -> Vect {
        Vect::from_slice(&self.to_array())
    }

    pub fn to_vect3(&self) -> Vect {
        Vect::from_slice(&[self.x, self.y, self.z])
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&mut self) -> Vect4 {
        let norm = self.norm();
        if norm == 0.0 {
            return self.clone();
        }
        self.x = self.x / norm;
        self.y = self.y / norm;
        self.z = self.z / norm;
        self.w = self.w / norm;
        self.clone()
    }

    pub fn dot(&self, other: &Vect4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Vect4) -> Vect4 {
        Vect4 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }
}

impl std::ops::Add<Vect4> for Vect4 {
    type Output = Vect4;

    fn add(self, other: Vect4) -> Vect4 {
        Vect4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl std::ops::Sub<Vect4> for Vect4 {
    type Output = Vect4;

    fn sub(self, other: Vect4) -> Vect4 {
        Vect4::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl std::ops::Mul<f32> for Vect4 {
    type Output = Vect4;

    fn mul(self, rhs: f32) -> Vect4 {
        Vect4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl std::ops::Neg for Vect4 {
    type Output = Vect4;

    fn neg(self) -> Vect4 {
        Vect4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::fmt::Display for Vect4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Clone for Vect4 {
    fn clone(&self) -> Vect4 {
        Vect4::new(self.x, self.y, self.z, self.w)
    }
}

