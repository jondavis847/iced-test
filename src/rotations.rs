pub mod quaternion;
pub mod rotation_matrix;
use quaternion::Quaternion;
use rotation_matrix::RotationMatrix;
#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    Quaternion(Quaternion),
    RotationMatrix(RotationMatrix),

}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::Quaternion(Quaternion::identity())
    }
}