use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EmbeddingType {
    TextEmbedding3Small,
    TextEmbedding3Large,
    Custom(usize),
}

impl EmbeddingType {
    pub fn dimension(&self) -> usize {
        match self {
            Self::TextEmbedding3Small => 1536,
            Self::TextEmbedding3Large => 3072,
            Self::Custom(dim) => *dim,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub id: String,
    pub vector: Vec<f32>,
    pub payload: HashMap<String, String>,
}

impl Point {
    pub fn new(vector: Vec<f32>, payload: HashMap<String, String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            vector,
            payload,
        }
    }

    pub fn with_id(id: String, vector: Vec<f32>, payload: HashMap<String, String>) -> Self {
        Self {
            id,
            vector,
            payload,
        }
    }
}
