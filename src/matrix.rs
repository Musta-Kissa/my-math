use core::f64;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut, Mul};

#[macro_export]
macro_rules! new_mat {
    ($rows:expr,$cols:expr) => {
        Matrix::<$rows, $cols>::new_zero();
    };
    ($rows:expr, $cols:expr, $slice:expr) => {
        Matrix::<$rows, $cols>::new(Vec::from($slice))
    };
}

#[derive(Clone, Copy)]
pub struct Matrix<const ROW: usize, const COL: usize> {
    data: [[f64; COL]; ROW],
}

impl<const ROW: usize, const COL: usize> Matrix<ROW, COL> {
    pub fn new(data: Vec<f64>) -> Self {
        assert_eq!(
            ROW * COL,
            data.len(),
            "Vector len doest match the matrix dimentions | vec len: {}, matrix dimentions: {}x{}", data.len() , ROW,COL
        );

        let mut tmp = Matrix::<ROW, COL>::new_zero();
        for (i, chunk) in data.chunks(COL).enumerate() {
            tmp[i].copy_from_slice(chunk);
        }
        tmp
    }
    fn new_zero() -> Self {
        Self {
            data: [[0.0; COL]; ROW],
        }
    }
}
impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut mat = Self::new_zero();
        for i in 0..N {
            mat[i][i] = 1.;
        }
        return mat;
    }
    pub fn scalar(factor: f64) -> Self {
        let mut mat = Self::new_zero();
        for i in 0..N {
            mat[i][i] = factor;
        }
        return mat;
    }
}

pub fn construct_proj(near: f64, far: f64, fov: f64, ratio: f64) -> Matrix<4, 4> {
    use std::f64::consts::PI;
    let fov_rad = fov / 180. * PI;

    let tan_half_fov = f64::tan(fov_rad / 2.);

    let top = tan_half_fov * near;
    let right = near * ratio * tan_half_fov;

    let n = near;
    let f = far;
    let r = right;
    let t = top;
    #[rustfmt::skip]
    let perspective_proj_mat = Matrix::<4,4>::new(
        vec![ n/r,  0.,    0.,       0.,
               0., n/t,    0.,       0.,
               0.,  0., f/(f-n), -(f*n)/(f-n),
               0.,  0.,    1.,       0.,     ]);
    return perspective_proj_mat;
}

use crate::vec::Vec3;
pub fn construct_camera_transform(camera_pos: Vec3, up: Vec3, at: Vec3) -> Matrix<4, 4> {
    let w = (camera_pos - at) / (camera_pos - at).mag();
    // negate the u vec if using a left hand system. Dont if using a right handed system
    let u = w.cross(up) / -w.cross(up).mag();
    // in a right hand coordinate system it would be w.cross(u)
    let v = u.cross(w);

    #[rustfmt::skip]
    let camera_transform = Matrix::<4,4>::new(vec![
                                u.x, u.y, u.z, -camera_pos.dot(u),
                                v.x, v.y, v.z, -camera_pos.dot(v),
                                w.x, w.y, w.z, -camera_pos.dot(w),
                                0.,  0.,  0.,        1.        ]);
    return camera_transform;
}

// ------------------- Traint Impls -----------------------------

impl<const ROW: usize, const COL: usize> Deref for Matrix<ROW, COL> {
    type Target = [[f64; COL]; ROW];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const ROW: usize, const COL: usize> DerefMut for Matrix<ROW, COL> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<const ROW: usize, const COL: usize, const ROW_RHS: usize, const COL_RHS: usize>
    Mul<Matrix<ROW_RHS, COL_RHS>> for Matrix<ROW, COL>
{
    type Output = Matrix<ROW, COL_RHS>;

    fn mul(self, rhs: Matrix<ROW_RHS, COL_RHS>) -> Self::Output {
        let mut mat: Self::Output = Matrix::<ROW, COL_RHS>::new_zero();

        for row in 0..ROW {
            for col in 0..COL_RHS {
                let mut sum: f64 = 0.0;
                for i in 0..COL {
                    sum += self[row][i] * rhs[i][col];
                }
                mat[row][col] = sum;
            }
        }
        mat
    }
}

impl<const ROW: usize, const COL: usize> Display for Matrix<ROW, COL> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 0 = whole number len | 1 = fraction len
        let mut longest_number_size_in_column = [[0; 2]; COL];
        for col in 0..COL {
            for row in 0..ROW {
                longest_number_size_in_column[col][0] = std::cmp::max(
                    longest_number_size_in_column[col][0],
                    format!("{}", self[row][col].floor()).len(),
                );
                if self[row][col].fract() == 0. {
                } else {
                    longest_number_size_in_column[col][1] = std::cmp::max(
                        longest_number_size_in_column[col][1],
                        format!("{:.3}", self[row][col].fract())
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .len()
                            - 1,
                    );
                }
            }
        }
        // Aligning numbers in the matrix and rouding go 3 digits after decimal point
        // [0.4,4]             [   0.4  ,  4    ]
        // [123,12.44]      => [ 123    , 12.44 ]
        // [1.11111111,2]      [   1.111,  3    ]

        for row in 0..ROW {
            write!(f, "[ ")?;
            for col in 0..COL {
                let whole_part_len = format!("{}", self[row][col].floor()).len();
                let fract_len: usize;
                if self[row][col].fract() == 0. {
                    fract_len = 0;
                } else {
                    fract_len = format!("{:.3}", self[row][col].fract())
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .len()
                        - 1;
                }

                let diff_front = longest_number_size_in_column[col][0] - whole_part_len;
                for _ in 0..diff_front {
                    write!(f, " ")?;
                }
                let no_trailing_zeros = format!("{:.3}", self[row][col])
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_owned();
                write!(f, "{}", no_trailing_zeros)?;
                let diff_back = longest_number_size_in_column[col][1] - fract_len;
                for _ in 0..diff_back {
                    write!(f, " ")?;
                }
                if col != COL - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, " ]\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec::*;

    macro_rules! print_ident {
        ($($size:expr),*) => { $(
            println!("{}x{} identity:\n{}",$size,$size,Matrix::<$size,$size>::identity());
            )*
        };
    }

    #[test]
    fn it_works() {
        //let mat_1 = Matrix::<2, 3>::new(Some(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let mat_1 = new_mat!(2, 3, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        println!("mat_1:\n{}", mat_1);
        let mat_2 = Matrix::<3, 2>::new(vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        println!("mat_2:\n{}", mat_2);
        println!("dot product:\n{}", mat_1 * mat_2);

        println!("mat_1 * 3x3itent:\n{}", mat_1 * Matrix::<3, 3>::identity());
        print_ident!(1, 2, 3, 4, 5, 6);
        println!(
            "matrix format test:\n{}",
            Matrix::<3, 2>::new(vec![0.4, 4., 123., 12.44, 1.1111111111, 3.])
        );
        let vec4 = Vec4::new(1.,2.,3.,1.);
        println!("vec4 * ident = {:?}", Matrix::<4, 4>::scalar(2.) * vec4);
    }
}
