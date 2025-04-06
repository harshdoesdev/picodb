use pikodb::embedding::EmbeddingType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IndexBuildQuality {
    Quick,
    Standard,
    Robust,
}

impl IndexBuildQuality {
    pub fn value(self) -> usize {
        match self {
            Self::Quick => 100,
            Self::Standard => 200,
            Self::Robust => 400,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct IndexConfig {
    pub build_quality: IndexBuildQuality,
    pub embedding_type: EmbeddingType,
}

impl IndexConfig {
    pub fn new(build_quality: IndexBuildQuality, embedding_type: EmbeddingType) -> Self {
        Self {
            build_quality,
            embedding_type,
        }
    }

    pub fn quick(embedding_type: EmbeddingType) -> Self {
        Self {
            build_quality: IndexBuildQuality::Quick,
            embedding_type,
        }
    }

    pub fn standard(embedding_type: EmbeddingType) -> Self {
        Self {
            build_quality: IndexBuildQuality::Standard,
            embedding_type,
        }
    }

    pub fn robust(embedding_type: EmbeddingType) -> Self {
        Self {
            build_quality: IndexBuildQuality::Robust,
            embedding_type,
        }
    }

    pub fn embedding_dimension(&self) -> usize {
        self.embedding_type.dimension()
    }
}
