use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Object not found: key={key}")]
    NotFound { key: String },

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] Box<dyn std::error::Error>),
}
