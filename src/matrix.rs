use core::f32;
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
    pub data: [[f32; COL]; ROW],
}

impl<const ROW: usize, const COL: usize> Matrix<ROW, COL> {
    pub fn new(data: Vec<f32>) -> Self {
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
    pub fn new_zero() -> Self {
        Self {
            data: [[0.0; COL]; ROW],
        }
    }
    pub fn to_opengl(self) -> Vec<f32> {
        let mut out = vec![];
        for col in 0..ROW {
            for row in 0..COL {
                out.push(self.data[row][col] as f32);
            }
        }
        out
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
    pub fn scalar(factor: f32) -> Self {
        let mut mat = Self::new_zero();
        for i in 0..N {
            mat[i][i] = factor;
        }
        return mat;
    }
}

use crate::vec::Vec3;
pub fn look_at_lh(camera_pos: Vec3, at: Vec3, up: Vec3) -> Matrix<4,4> {

    let dir = camera_pos - at;

    let f = dir.norm();
    let s = f.cross(up).norm();
    let u = s.cross(f);

    Matrix::<4,4>::new(vec![
         s.x, s.y, s.z, -camera_pos.dot(s),
         u.x, u.y, u.z, -camera_pos.dot(u),
         f.x, f.y, f.z, -camera_pos.dot(f),
         0.,  0.,  0.,   1.,
    ])
}

pub fn proj_mat_gl(fov: f32, ratio: f32, near: f32,far: f32) -> Matrix<4,4> {
    use std::f32::consts::PI;
    let fov_rad = fov / 180. * PI;
    let tan_half_fov = f32::tan(fov_rad / 2.);

    Matrix::<4,4>::new(vec![
        1./(tan_half_fov*ratio),    0., 0., 0.,
        0., 1./tan_half_fov,         0., 0.,
        0., 0., (far+near)/(near-far),(2.*near*far)/(near-far),
        0., 0.,  -1.   , 0.,
    ])
}
pub fn proj_mat_wgpu(fov: f32, ratio: f32, near: f32,far: f32) -> Matrix<4,4> {
    use std::f32::consts::PI;
    let fov_rad = fov / 180. * PI;
    let tan_half_fov = f32::tan(fov_rad / 2.);

    let h_c = (far+near)/((near-far)*2.);
    let h_d = (near*far)/(near-far);
    Matrix::<4,4>::new(vec![
        1./(tan_half_fov*ratio),    0.,         0.,     0.,
                0.,          1./tan_half_fov,   0.,     0.,
                0.,                 0.,         h_c,    h_d,
                0.,                 0.,         h_c-1., h_d,
    ])
}

// ------------------- Traint Impls -----------------------------

impl<const ROW: usize, const COL: usize> Deref for Matrix<ROW, COL> {
    type Target = [[f32; COL]; ROW];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const ROW: usize, const COL: usize> DerefMut for Matrix<ROW, COL> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<const ROW: usize, const COL: usize> Mul<f32> for Matrix<ROW, COL> {
    type Output = Matrix<ROW,COL>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut out = Matrix::new_zero();
        for r in 0..ROW {
            for c in 0..COL {
                out[r][c] = self[r][c] * rhs;
            }
        }
        out
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
                let mut sum: f32 = 0.0;
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

use crate::fraction::Fraction;
use crate::fr;

#[derive(Clone, Copy)]
pub struct MatrixFr<const ROW: usize, const COL: usize> {
    pub data: [[Fraction; COL]; ROW],
}
impl<const ROW: usize, const COL: usize> MatrixFr<ROW, COL> {
    pub fn new(data: Vec<Fraction>) -> Self {
        assert_eq!(
            ROW * COL,
            data.len(),
            "Vector len doest match the matrix dimentions | vec len: {}, matrix dimentions: {}x{}", data.len() , ROW,COL
        );

        let mut tmp = MatrixFr::<ROW, COL>::new_zero();
        for (i, chunk) in data.chunks(COL).enumerate() {
            tmp[i].copy_from_slice(chunk);
        }
        tmp
    }
    fn new_zero() -> Self {
        Self {
            data: [[fr!(0); COL]; ROW],
        }
    }
    pub fn solve(&mut self) -> [Fraction;ROW] {
        assert_eq!(ROW + 1,COL,"You can only solve a Nx(N+1) matrix");
        for i in 0..ROW{
            // Find the pivot element
            let mut max_row = i;
            for k in i + 1..ROW{
                if self[k][i].abs() > self[max_row][i].abs() {
                    max_row = k;
                }
            }

            // Swap rows if necessary
            self.swap(i, max_row);

            // Check if the pivot is zero (if so, the matrix is singular)
            if self[i][i] == 0 {
                panic!("singular matrix"); // No unique solution
            }

            // Eliminate below the pivot
            for j in i + 1..ROW {
                let factor = self[j][i] / self[i][i];
                for k in i..ROW + 1 {
                    self[j][k] = self[j][k] - factor * self[i][k];
                }
            }
        }

        // Back substitution
        let mut solution = [fr!(0); ROW];

        for i in (0..ROW).rev() {
            solution[i] = self[i][ROW] / self[i][i];
            for j in 0..i {
                self[j][ROW] = self[j][ROW] - self[j][i] * solution[i];
            }
        }
        solution
    }
}
impl<const ROW: usize, const COL: usize> Deref for MatrixFr<ROW, COL> {
    type Target = [[Fraction; COL]; ROW];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const ROW: usize, const COL: usize> DerefMut for MatrixFr<ROW, COL> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<const ROW: usize, const COL: usize> Mul<Fraction> for MatrixFr<ROW, COL> {
    type Output = MatrixFr<ROW,COL>;

    fn mul(self, rhs: Fraction) -> Self::Output {
        let mut out = MatrixFr::new_zero();
        for r in 0..ROW {
            for c in 0..COL {
                out[r][c] = self[r][c] * rhs;
            }
        }
        out
    }
}

impl<const ROW: usize, const COL: usize, const ROW_RHS: usize, const COL_RHS: usize>
    Mul<MatrixFr<ROW_RHS, COL_RHS>> for MatrixFr<ROW, COL>
{
    type Output = MatrixFr<ROW, COL_RHS>;

    fn mul(self, rhs: MatrixFr<ROW_RHS, COL_RHS>) -> Self::Output {
        let mut mat: Self::Output = MatrixFr::<ROW, COL_RHS>::new_zero();

        for row in 0..ROW {
            for col in 0..COL_RHS {
                let mut sum: Fraction = fr!(0);
                for i in 0..COL {
                    sum += self[row][i] * rhs[i][col];
                }
                mat[row][col] = sum;
            }
        }
        mat
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
