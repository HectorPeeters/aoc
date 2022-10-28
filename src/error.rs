use thiserror::Error;

pub type Result<T> = std::result::Result<T, AocError>;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Try from int error: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),

    #[error("Error: {0}")]
    User(String),
}
