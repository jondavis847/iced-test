use std::ops::Mul;
use crate::linear_algebra::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct RotationMatrix {
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

impl RotationMatrix {
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
        // check that each column is normalized to magnitude 1.0
        let check_normalized = |e1:f64,e2:f64,e3:f64| {
            let mag_squared = e1*e1 + e2*e2 + e3*e3;            
            // divide by 0.0 protection
            assert!(mag_squared > f64::EPSILON, "RotationMatrix column vector had 0 magnitude");

            if !((mag_squared-1.0).abs() < f64::EPSILON) {
                let mag = mag_squared.sqrt();
                
                let e1 = e1/mag;
                let e2 = e2/mag;
                let e3 = e3/mag;
            } 
            (e1,e2,e3)
        };

        let (e11,e21,e31) = check_normalized(e11,e21,e31);
        let (e12,e22,e32) = check_normalized(e12,e22,e32);
        let (e13,e23,e33) = check_normalized(e13,e23,e33);

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

impl Mul<Vector3> for RotationMatrix {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3{
        Vector3::new(            
            self.e11 * rhs.e1 + self.e12 * rhs.e2 + self.e13 * rhs.e3,
            self.e21 * rhs.e1 + self.e22 * rhs.e2 + self.e23 * rhs.e3,
            self.e31 * rhs.e1 + self.e32 * rhs.e2 + self.e33 * rhs.e3,
        )
    }
}