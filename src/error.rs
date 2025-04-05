use thiserror::Error;

#[derive(Error, Debug)]
pub enum VectorDbError {
    #[error("Embedding dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("Collection '{name}' not found")]
    CollectionNotFound { name: String },
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("Serialization failed: {0}")]
    Serialization(#[source] bincode::Error),
    #[error("Deserialization failed: {0}")]
    Deserialization(#[source] bincode::Error),
    #[error("File operation failed: {0}")]
    FileOperation(#[source] std::io::Error),
    #[error("Persistence adapter not configured")]
    NotConfigured,
}
