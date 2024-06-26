use std::ops::Mul;
use rand::prelude::*;  
use crate::linear_algebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {    
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub s: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, s: f64) -> Self {        
        let mag_squared = x*x + y*y + z*z + s*s;
        let mag = mag_squared.sqrt();

        assert!(mag>0.0, "Values produced 0.0 magnitude Quaternion, [{}, {}, {}, {}]", x,y,z,s);

        Self {x: x/mag, y: y/mag, z: z/mag, s: s/mag }
    }

    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn rand() -> Self {
            let mut rng = thread_rng();
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            let z = rng.gen_range(-1.0..1.0);
            let s = rng.gen_range(-1.0..1.0);
            
            Self::new(x,y,z,s)        
        }

    pub fn inv(&self) -> Self {
            Self::new(-self.x,-self.y,-self.z,self.s)
        }    

    pub fn transform(&self, v: Vector3) -> Vector3 {
        let v_mag = v.norm();        
        let v_augmented = Self::new(v.e1,v.e2,v.e3,0.0);
        let inv_q = self.inv();
        let q_tmp = inv_q * v_augmented;
        let q_tmp = q_tmp * *self;
        Vector3::new(v_mag * q_tmp.x,v_mag * q_tmp.y,v_mag * q_tmp.z )
    }

    pub fn rotate(&self, v: Vector3) -> Vector3 {
        let v_mag = v.norm();        
        let v_augmented = Self::new(v.e1,v.e2,v.e3,0.0);
        let inv_q = self.inv();
        let q_tmp = *self * v_augmented;
        let q_tmp = q_tmp * inv_q;
        Vector3::new(v_mag * q_tmp.x,v_mag * q_tmp.y,v_mag * q_tmp.z )
    }

}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self{
        Self::new(            
            self.s * rhs.x + self.x * rhs.s + self.y * rhs.z - self.z * rhs.y,
            self.s * rhs.y - self.x * rhs.z + self.y * rhs.s + self.z * rhs.x,
            self.s * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.s,
            self.s * rhs.s - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_quaternion_normalization() {        

        let q = Quaternion::new(1.0,2.0,3.0,4.0);        

        assert_eq!(q.x, 0.18257418583505536);
        assert_eq!(q.y, 0.3651483716701107);
        assert_eq!(q.z, 0.5477225575051661);
        assert_eq!(q.s, 0.7302967433402214);
    }

    #[test]
    fn test_quaternion_inv() {        

        let quat = Quaternion::rand();
        let inv = quat.inv();

        assert_eq!(inv.s, quat.s);
        assert_eq!(inv.x, -quat.x);
        assert_eq!(inv.y, -quat.y);
        assert_eq!(inv.z, -quat.z);
    }

    #[test]
    fn test_quaternion_multiplication() {
        let quat1 = Quaternion::new(0.18119546436307749, 0.4381103371317225, 0.10015469662419728, 0.8747551502773175);
        let quat2 = Quaternion::new( 0.4605004692970668, -0.13901506620501594, -0.7574522634418864, -0.44145237314115715);
        let result = quat1 * quat2;

        assert_eq!(result.s, -0.3328369942072665);
        assert_eq!(result.x, 0.004911334761481288);
        assert_eq!(result.y,-0.13164079374848636);
        assert_eq!(result.z, -0.9337377123685223);
    }
}