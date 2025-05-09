use core::f32;
use std::ops::{Add,Div,Sub, Mul};
use std::f32::consts::PI;

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
    pub x: i32,
    pub y: i32,
}
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        IVec2 { x, y }
    }
    pub fn cross(&self, rhs: Self) -> i32 {
        self.x * rhs.y - self.y * rhs.x
    }
}
impl Add<IVec2> for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<IVec2> for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: IVec2) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr,$y:expr) => {
        Vec2 { x: $x, y: $y }
    };
}
#[derive(Copy,Clone,Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
    pub fn cross(&self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }
    pub fn mag(&self) -> f32 {
        let x = self.x;
        let y = self.y;

        f32::sqrt(x*x + y*y)
    }
    pub fn norm(&self) -> Vec2 {
        *self / self.mag()
    }
}
impl Div<f32> for Vec2 {
    type Output = Vec2;
    
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut out = Vec2::new(0.,0.);
            out.x = self.x * rhs;
            out.y = self.y * rhs;
        out
    }
}
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let mut out = Vec2::new(0.,0.);
            out.x = self * rhs.x;
            out.y = self * rhs.y;
        out
    }
}
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[macro_export]
macro_rules! ivec3 {
    ($x:expr,$y:expr,$z:expr) => {
        IVec3 { x: $x as i32, y: $y as i32, z: $z as i32 }
    };
}
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl IVec3 {
    pub const ZERO: IVec3 = IVec3 { x: 0 , y: 0 , z: 0 };

    pub const X:    IVec3 = IVec3 { x: 1 , y: 0 , z: 0 };
    pub const NEG_X:IVec3 = IVec3 { x: -1 , y: 0 , z: 0 };
    pub const Y:    IVec3 = IVec3 { x: 0 , y: 1 , z: 0 };
    pub const NEG_Y:IVec3 = IVec3 { x: 0 , y: -1 , z: 0 };
    pub const Z:    IVec3 = IVec3 { x: 0 , y: 0 , z: 1 };
    pub const NEG_Z:IVec3 = IVec3 { x: 0 , y: 0 , z: -1 };

    pub fn new(x:i32,y:i32,z:i32) -> Self {
        IVec3{ x, y, z }
    }

    pub fn modulo(&self, rhs: i32) -> Self {
        IVec3 {
            x: (self.x%rhs + rhs) % rhs,
            y: (self.y%rhs + rhs) % rhs,
            z: (self.z%rhs + rhs) % rhs,
        }
    }
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32,  self.y as f32, self.z as f32)
    }
    pub fn div_floor(&self, rhs: i32) -> Self {
        fn div_floor(a:i32,b:i32) -> i32 {
            let quotient = a / b;
            let remainder = a % b;
            if remainder != 0 && ((a < 0) ^ (b < 0)) {
                quotient - 1
            } else {
                quotient
            }
        }
        IVec3 {
            x: div_floor(self.x,rhs),
            y: div_floor(self.y,rhs),
            z: div_floor(self.z,rhs),
        }
    }
}
impl Mul<i32> for IVec3 {
    type Output = IVec3;

    fn mul(self, rhs: i32) -> Self::Output {
        IVec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Add<i32> for IVec3 {
    type Output = IVec3;

    fn add(self, rhs: i32) -> Self::Output {
        IVec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl Add<IVec3> for IVec3 {
    type Output = IVec3;

    fn add(self, rhs: IVec3) -> Self::Output {
        IVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub<IVec3> for IVec3 {
    type Output = IVec3;

    fn sub(self, rhs: IVec3) -> Self::Output {
        IVec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Div<i32> for IVec3 {
    type Output = IVec3;

    fn div(self, rhs: i32) -> Self::Output {
        IVec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Into<Vec3> for IVec3 {
    fn into(self) -> Vec3 {
        Vec3 { x: self.x as f32, 
               y: self.y as f32, 
               z: self.z as f32
        }
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr,$y:expr,$z:expr) => {
        Vec3::new($x,$y,$z)
    };
    ($val:expr) => {
        Vec3::new($val,$val,$val);
    }
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const UP:Vec3 = Vec3 { x: 0. , y: 1. , z: 0. };
    pub const ZERO:Vec3 = Vec3 { x: 0. , y: 0. , z: 0. };

    pub const X:    Vec3 = Vec3 { x: 1. , y: 0. , z: 0. };
    pub const NEG_X:Vec3 = Vec3 { x: -1. , y: 0. , z: 0. };
    pub const Y:    Vec3 = Vec3 { x: 0. , y: 1. , z: 0. };
    pub const NEG_Y:Vec3 = Vec3 { x: 0. , y: -1. , z: 0. };
    pub const Z:    Vec3 = Vec3 { x: 0. , y: 0. , z: 1. };
    pub const NEG_Z:Vec3 = Vec3 { x: 0. , y: 0. , z: -1. };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn from_slice(slice: [f32;3]) -> Vec3 {
        Vec3::new(slice[0], slice[1], slice[2])
    }
    pub fn mag(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        f32::sqrt(x*x + y*y + z*z)
    }
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn dot(&self, rhs: Self) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
    pub fn with_x(&self, x: f32) -> Self {
        Vec3 { x, y: self.y, z: self.z }  
    }
    pub fn with_y(&self, y: f32) -> Self {
        Vec3 { x: self.x , y, z: self.z }  
    }
    pub fn with_z(&self, z: f32) -> Self {
        Vec3 { x: self.x , y: self.y, z }  
    }
    /// takes in a deg and a normalized axis vector 
    pub fn rot_quat(&mut self, deg: f32, axis: Vec3) {
        let deg_rad = deg / 180. * PI;
        let rot_quat = Quaternion::new(f32::cos(deg_rad/2.),axis * f32::sin(deg_rad/2.));
        let vec_quat = Quaternion::new(0.,*self);
        let out_vec_quat = rot_quat * vec_quat * rot_quat.conjugate();
        *self = vec3!(out_vec_quat.v.x,out_vec_quat.v.y,out_vec_quat.v.z);
    }
    pub fn norm(&self) -> Vec3 {
        *self / self.mag()
    }
    pub fn to_vec4(&self,w:f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
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
impl Div<Vec3> for f32 {
    type Output = Vec3;
    
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        let mut out = Vec3::new(0.,0.,0.);
            out.x = self.x + rhs;
            out.y = self.y + rhs;
            out.z = self.z + rhs;
        out
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut out = Vec3::new(0.,0.,0.);
            out.x = self.x * rhs;
            out.y = self.y * rhs;
            out.z = self.z * rhs;
        out
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<Vec3> for f32 {
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

#[macro_export]
macro_rules! vec4 {
    ($x:expr,$y:expr,$z:expr,$w:expr) => {
        Vec4::new($x,$y,$z,$w)
    };
}

#[derive(Debug,Copy,Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
    pub fn from_slice(slice: &[f32]) -> Vec4 {
        match slice.len() {
            1 => Vec4::new(slice[0], 1., 1., 1.),
            2 => Vec4::new(slice[0], slice[1], 1., 1.),
            3 => Vec4::new(slice[0], slice[1], slice[2], 1.),
            4 => Vec4::new(slice[0], slice[1], slice[2], slice[3]),
            _ => Vec4::new(1., 1., 1., 1.),
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl Div<f32> for Vec4 {
    type Output = Vec4;
    
    fn div(self, rhs: f32) -> Self::Output {
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

