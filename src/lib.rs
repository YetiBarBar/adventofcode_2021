use thiserror::Error;
pub mod matrix;
pub mod submarine;
pub mod utils;
pub use crate::matrix::Matrix2D;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Incorrect parsing")]
    ParsingError,
}
