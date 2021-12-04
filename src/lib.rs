use thiserror::Error;

pub mod submarine;
pub mod utils;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Incorrect parsing")]
    ParsingError,
}
