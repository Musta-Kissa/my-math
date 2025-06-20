use std::ops::Mul;

use crate::vec::Vec3;
use crate::vec3;

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub v: Vec3,
}
impl Quaternion {
    pub fn new(w: f32,v:Vec3) -> Self{
        Quaternion {
            w,v,
        }
    }
    /// Works only if the Quaternion is normalized
    pub fn conjugate(&self) -> Self {
        Quaternion::new(self.w , self.v * -1.)
    }
    pub fn xyz(&self) -> Vec3 {
        vec3!(self.v.x,self.v.y,self.v.z)
    }
}
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            w: self.w * rhs.w - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z,
            v: Vec3 {
                x: self.w * rhs.v.x + self.v.x * rhs.w + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
                y: self.w * rhs.v.y - self.v.x * rhs.v.z + self.v.y * rhs.w + self.v.z * rhs.v.x,
                z: self.w * rhs.v.z + self.v.x * rhs.v.y - self.v.y * rhs.v.x + self.v.z * rhs.w,
            }
        }
    }
}

/// Assuming the quaternion is normalized
pub fn rot_vec_by_quat(vec: Vec3,quat:&Quaternion) -> Vec3 {
    let vec_quat = Quaternion::new(0.,vec);
    let out_vec_quat = (*quat) * vec_quat * quat.conjugate();
    out_vec_quat.xyz()
}
