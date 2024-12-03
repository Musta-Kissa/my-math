use core::f64;
use std::ops::{Add,Div,Sub, Mul};
use std::f64::consts::PI;

use super::quat::Quaternion;
use super::matrix::Matrix;

#[macro_export]
macro_rules! ivec2 {
    ($x:expr,$y:expr) => {
        IVec2 { x: $x, y: $y }
    };
}

#[derive(Debug,Clone, Copy)]
pub struct IVec2 {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y}
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr,$y:expr,$z:expr) => {
        Vec3::new($x,$y,$z)
    };
}

#[derive(Debug,Copy,Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn mag(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        f64::sqrt(x*x + y*y + z*z)
    }
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn dot(&self, rhs: Self) -> f64 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
    pub fn rot_x(&mut self, deg: f64) {
        let deg_rad = deg / 180. * PI;
        #[rustfmt::skip]
        let rot_mat = Matrix::<3,3>::new(vec![
            f64::cos(deg_rad), f64::sin(deg_rad),0.,
            -f64::sin(deg_rad),  f64::cos(deg_rad),0.,
            0.,0.,1.]);
        println!("rotating x: {}",deg);
        *self = rot_mat * (*self);
    }
    pub fn rot_y(&mut self,deg: f64) {
        let deg_rad = deg / 180. * PI;
        #[rustfmt::skip]
        let rot_mat = Matrix::<3,3>::new(vec![
            f64::cos(deg_rad), 0.,-f64::sin(deg_rad),
            0.,1.,0.,
            f64::sin(deg_rad),  0.,f64::cos(deg_rad) ]);
        println!("rotating y: {}",deg);
        *self = rot_mat * (*self);
    }
    pub fn rot_z(&mut self,deg: f64) {
        let deg_rad = deg / 180. * PI;

        #[rustfmt::skip]
        let rot_mat = Matrix::<3,3>::new(vec![
            1.,     0.,                 0.,
            0., f64::cos(deg_rad), f64::sin(deg_rad),
            0.,-f64::sin(deg_rad), f64::cos(deg_rad), ]);
        println!("rotating z: {}",deg);
        *self = rot_mat * (*self);
    }
    /// takes in a deg and a normalized axis vector 
    pub fn rot_quat(&mut self, deg: f64, axis: Vec3) {
        let deg_rad = deg / 180. * PI;
        let rot_quat = Quaternion::new(f64::cos(deg_rad),axis * f64::sin(deg_rad));
        let vec_quat = Quaternion::new(0.,*self);
        let out_vec_quat = rot_quat * vec_quat * rot_quat.conjugate();
        *self = vec3!(out_vec_quat.v.x,out_vec_quat.v.y,out_vec_quat.v.z);
    }
    pub fn norm(&self) -> Vec3 {
        *self / self.mag()
    }
}
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = Vec3::new(0.,0.,0.);
            out.x = self.x * rhs;
            out.y = self.y * rhs;
            out.z = self.z * rhs;
        out
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut out = Vec3::new(0.,0.,0.);
            out.x = self * rhs.x;
            out.y = self * rhs.y;
            out.z = self * rhs.z;
        out
    }
}
impl Mul<Vec3> for Matrix<3, 3> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut out = Vec3::new(0.,0.,0.);
            out.x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z;
            out.y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z;
            out.z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z;
        out
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4 { x, y, z, w }
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;
    
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Mul<Vec4> for Matrix<4, 4> {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut out = Vec4::new(0.,0.,0.,0.);
            out.x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w;
            out.y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w;
            out.z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w;
            out.w = self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w;
        out
    }
}

