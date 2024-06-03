use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
}

impl Vector3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e1, e2, e3 }
    }

    pub fn norm(&self) -> f64 {
        (self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.norm();
        Vector3::new(self.e1 / mag, self.e2 / mag, self.e3 / mag)
    }

    pub fn skew(&self) -> Matrix3 {
        Matrix3::new(
            0.0,
            self.e3,
            -self.e2,
            -self.e3,
            0.0,
            self.e1,
            self.e2,
            -self.e1,
            0.0
        )
    }

    /// self x rhs
    pub fn cross(&self, rhs:Self) -> Self {
        Self::new(
            self.e2 * rhs.e3 - self.e3 * rhs.e2,
            self.e3 * rhs.e1 - self.e1 * rhs.e3,
            self.e1 * rhs.e2 - self.e2 * rhs.e1
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix3 {
    e11: f64,
    e21: f64,
    e31: f64,
    e12: f64,
    e22: f64,
    e32: f64,
    e13: f64,
    e23: f64,
    e33: f64,
}
impl Matrix3 {
    fn new(
        e11: f64,
        e21: f64,
        e31: f64,
        e12: f64,
        e22: f64,
        e32: f64,
        e13: f64,
        e23: f64,
        e33: f64,
    ) -> Self {
        Self {
            e11,
            e21,
            e31,
            e12,
            e22,
            e32,
            e13,
            e23,
            e33,
        }
    }
}


impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3{
        Vector3::new(            
            self.e11 * v.e1 + self.e12 * v.e2 + self.e13 * v.e3,
            self.e21 * v.e1 + self.e22 * v.e2 + self.e23 * v.e3,
            self.e31 * v.e1 + self.e32 * v.e2 + self.e33 * v.e3,
        )
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self{
        Self::new(            
            self.e11 * rhs.e11 + self.e12 * rhs.e21 + self.e13 * rhs.e31,
            self.e21 * rhs.e11 + self.e22 * rhs.e21 + self.e23 * rhs.e31,
            self.e31 * rhs.e11 + self.e32 * rhs.e21 + self.e33 * rhs.e31,
            self.e11 * rhs.e12 + self.e12 * rhs.e22 + self.e13 * rhs.e32,
            self.e21 * rhs.e12 + self.e22 * rhs.e22 + self.e23 * rhs.e32,
            self.e31 * rhs.e12 + self.e32 * rhs.e22 + self.e33 * rhs.e32,
            self.e11 * rhs.e13 + self.e12 * rhs.e23 + self.e13 * rhs.e33,
            self.e21 * rhs.e13 + self.e22 * rhs.e23 + self.e23 * rhs.e33,
            self.e31 * rhs.e13 + self.e32 * rhs.e23 + self.e33 * rhs.e33,
        )
    }
}