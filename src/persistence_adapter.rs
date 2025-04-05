use pikodb::{collection::CollectionData, error::VectorDbError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    pub collections: HashMap<String, CollectionData>,
}

pub trait PersistenceAdapter {
    fn save(&self, state: &PersistedState) -> Result<(), VectorDbError>;
    fn load(&self) -> Result<PersistedState, VectorDbError>;
}

pub struct FsPersistenceAdapter {
    path: PathBuf,
}

impl FsPersistenceAdapter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl PersistenceAdapter for FsPersistenceAdapter {
    fn save(&self, state: &PersistedState) -> Result<(), VectorDbError> {
        let encoded = bincode::serialize(state)
            .map_err(|e| VectorDbError::PersistenceError(e.to_string()))?;
        std::fs::write(&self.path, encoded)
            .map_err(|e| VectorDbError::PersistenceError(e.to_string()))?;
        Ok(())
    }

    fn load(&self) -> Result<PersistedState, VectorDbError> {
        let mut file = std::fs::File::open(&self.path)
            .map_err(|e| VectorDbError::PersistenceError(e.to_string()))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| VectorDbError::PersistenceError(e.to_string()))?;
        let decoded = bincode::deserialize(&buf)
            .map_err(|e| VectorDbError::PersistenceError(e.to_string()))?;
        Ok(decoded)
    }
}
