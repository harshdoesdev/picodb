use thiserror::Error;

#[derive(Error, Debug)]
pub enum VectorDbError {
    #[error("Embedding dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("Collection '{name}' not found")]
    CollectionNotFound { name: String },
    #[error("Persistence error: {0}")]
    PersistenceError(String),
}
