pub mod fraction;
pub mod matrix;
pub mod quat;
pub mod vec;

pub mod prelude {
    pub use crate::fraction;
    pub use crate::fraction::*;
    pub use crate::matrix;
    pub use crate::matrix::*;
    pub use crate::quat;
    pub use crate::quat::*;
    pub use crate::vec;
    pub use crate::vec::*;
}
