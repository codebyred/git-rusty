use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitObjectError{
    #[error("Invalid hash length: expected <= 40 hex characters, got {0}")]
    InvalidHashLength(usize)

}