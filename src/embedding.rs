use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EmbeddingType {
    TextEmbedding3Small,
    TextEmbedding3Large,
}

impl EmbeddingType {
    pub fn dimension(&self) -> usize {
        match self {
            Self::TextEmbedding3Small => 1536,
            Self::TextEmbedding3Large => 3072,
        }
    }
}

pub type Metadata = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub id: Uuid,
    pub vector: Vec<f32>,
    pub metadata: Option<Metadata>,
}

impl Point {
    pub fn new(vector: Vec<f32>, metadata: Option<Metadata>) -> Self {
        Self {
            id: Uuid::new_v4(),
            vector,
            metadata,
        }
    }

    pub fn with_id(id: Uuid, vector: Vec<f32>, metadata: Option<Metadata>) -> Self {
        Self {
            id,
            vector,
            metadata,
        }
    }
}
