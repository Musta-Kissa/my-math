## Description 
This is a collection of math utilities that i made for my projects

### Usage
Cargo.toml
```Toml
[dependencies]
minifb = "0.27.0"
my_math = { path = "../my-math" }
```
src/main.rs
```Rust
use my_math::prelude::*;

fn main() {
	let vec = ivec2!(53,53);
	let mat = new_mat!(2,1,[3.,1.]);
	let frac = fr!(14,15);
}
```

### Matrices
#### Usage
```Rust
use my_math::matrix::*;

fn main() {
	let mat = Matrix::<3,3>::new(vec![2.,3.,4.,
									  5.,6.,7.,
									  9.,0.,1.]);
	// OR
	let mat = new_mat!(2,3,[1.,2.,3.,4.,5.,6.]);

	println!("{}", mat * 3.);
}
```


| **Methods**     | ``new()``<br>``new_zero()``                                                                                                    |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Methods NxN** | ``identity()``<br>``scalar()``                                                                                                 |
| **Utils**       | `construct_proj()`<br>`construct_camera_transform()`                                                                           |
| **Impl's**      | `Deref`<br>`DerefMut`<br>`Mul<Matrix> for Matrix`<br>`Mul<Vec3> for Matrix`<br>`Mul<Vec4> for Matrix`<br> `Display for Matrix` |

**Utils**
- `construct_proj()` - constructs a projection matrix given the `near`, `far` plane, `fov`, and the aspect `ratio`
- `construct_camera_transform()` - constructs a camera transformation matrix given a `camera_pos`, `up` direction and a `at` vector

-----
### Vectors
It Provides the following vector types:

|           | Macros     | Methods                                                                                                           | Impl's                                                                                                                                                         |
| --------- | ---------- | ----------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **IVec2** | `ivec2!()` | `new()`                                                                                                           | -----                                                                                                                                                          |
| **Vec3**  | `vec3!()`  | ``new()``<br>``mag()``<br>``cross()``<br>``dot()``<br>``rot_x()``<br>``rot_y()``<br>``rot_z()``<br>``rot_quat()`` | ``Add<Vec3> for Vec3``<br>``Sub<Vec3> for Vec3``<br>``Div<f64> for Vec3``<br>``Mul<f64> for Vec3``<br>``Mul<Vec3> for f64 ``<br>``Mul<Vec3> for Matrix<3, 3>`` |
| **Vec4**  | ----       | `new()`                                                                                                           | ``Div<f64> for Vec4``<br>``Mul<Vec4> for Matrix<4, 4> ``                                                                                                       |

------
### Quaterions
**Methods**:
```
new()
conjugate()
```
**Impl's**:
`Mul<Quaternion> for Quaternion`

-----
### Fractions
#### How to use
```rust
use my_fraction::*;

fn main() {
    println!("{}",fr!(2,3));
}
```
#### What it provides
##### Macro
It provides a macro `fr!(2,3)` that is a shorthand for `MyFraction::new(2,3)` 
##### Methods
There are two public methods provided:
- `as_f64()` that returns the approximation of the fraction as a `f64` type
- `as_mixed()` that returns a string of the fraction written in the [mixed numbers](https://en.wikipedia.org/wiki/Fraction#Mixed_numbers) representation 
##### Traits
This type provides implementations for these Traits:

The `$type` stands for all of these types: `usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128`
```Rust 
impl Sub<MyFraction> for MyFraction {...}
impl Sub<$type> for MyFraction {...}
impl Sub<MyFraction> for $type {...}

impl Add<MyFraction> for MyFraction {...}
impl Add<$type> for MyFraction {...}
impl Add<MyFraction> for $type {...}

impl Mul<MyFraction> for MyFraction {...}
impl Mul<$type> for MyFraction {...}
impl Mul<MyFraction> for $type {...}

impl Div<MyFraction> for MyFraction {...}
impl Div<MyFraction> for $type {...}
impl Div<$type> for MyFraction {...}

impl SubAssign for MyFraction {...}
impl AddAssign for MyFraction {...}
impl MulAssign for MyFraction {...}
impl DivAssign for MyFraction {...}

impl SubAssign<$type> for MyFraction {...}
impl AddAssign<$type> for MyFraction {...}
impl MulAssign<$type> for MyFraction {...}
impl DivAssign<$type> for MyFraction {...}

impl PartialEq for MyFraction {...}
impl PartialOrd for MyFraction {...}
impl Ord for MyFraction {...}

impl PartialEq<$type> for MyFraction {...}
impl PartialOrd<$type> for MyFraction {...}
impl PartialEq<MyFraction> for $type {...}
impl PartialOrd<MyFraction> for $type {...}

impl From<$type> for MyFraction {...}

impl Display for MyFraction {...}
```
#### Examples
```rust
use my_math::fraction::*;

fn main() {
    let mut fr = MyFraction::new(7, 3);
    println!("{fr}"); //(7/3)

    println!("{}", fr.as_f64());   //2.33333333333
    println!("{}", fr.as_mixed()); //2(1/3)

    fr += 7;        println!("{}", fr); //(28/3)
    fr -= 2;        println!("{}", fr); //(22/3)
    fr = fr * 5;    println!("{}", fr); //(110/3)
    fr *= fr!(2,1); println!("{}", fr); //(220/3)
    fr *= 3;        println!("{}", fr); //220
    fr = fr / 2;    println!("{}", fr); //110
    fr /= 4;        println!("{}", fr); //(55/2)
    fr = fr * 0;    println!("{}", fr); //0
    fr = fr + 20;    println!("{}", fr);//20

    println!("{}", fr!(3,2) < 9);          //true
    println!("{}", fr!(3,2) < fr!(1, 2));  //false
    println!("{}", fr!(3,2) == fr!(3, 2)); //true
}
```
