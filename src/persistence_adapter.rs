use pikodb::{
    error::{PersistenceError, VectorDbError},
    state::CollectionState,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub trait PersistenceAdapter {
    fn save(&self, state: &PersistedState) -> Result<(), VectorDbError>;
    fn load(&self) -> Result<PersistedState, VectorDbError>;
}

#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    pub collections: HashMap<String, CollectionState>,
}

pub struct FileSystemPersistenceAdapter {
    path: PathBuf,
}

impl FileSystemPersistenceAdapter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl PersistenceAdapter for FileSystemPersistenceAdapter {
    fn save(&self, state: &PersistedState) -> Result<(), VectorDbError> {
        let encoded = bincode::serialize(state)
            .map_err(|e| VectorDbError::PersistenceError(PersistenceError::Serialization(e)))?;
        std::fs::write(&self.path, encoded)
            .map_err(|e| VectorDbError::PersistenceError(PersistenceError::FileOperation(e)))?;
        Ok(())
    }

    fn load(&self) -> Result<PersistedState, VectorDbError> {
        let buf = std::fs::read(&self.path)
            .map_err(|e| VectorDbError::PersistenceError(PersistenceError::FileOperation(e)))?;
        let decoded = bincode::deserialize(&buf)
            .map_err(|e| VectorDbError::PersistenceError(PersistenceError::Deserialization(e)))?;
        Ok(decoded)
    }
}

pub enum Persistence {
    Filesystem(FileSystemPersistenceAdapter),
}

impl Persistence {
    pub fn filesystem(path: PathBuf) -> Self {
        Persistence::Filesystem(FileSystemPersistenceAdapter::new(path))
    }

    pub fn save(&self, state: &PersistedState) -> Result<(), VectorDbError> {
        match self {
            Persistence::Filesystem(adapter) => adapter.save(state),
        }
    }

    pub fn load(&self) -> Result<PersistedState, VectorDbError> {
        match self {
            Persistence::Filesystem(adapter) => adapter.load(),
        }
    }
}
