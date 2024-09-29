use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Object not found: key={0}")]
    NotFound(String),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] Box<dyn std::error::Error + Sync + Send>),
}
