use hnsw_rs::prelude::*;
use pikodb::{
    embedding::{EmbeddingType, Point},
    error::VectorDbError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CollectionData {
    pub embedding_type: EmbeddingType,
    pub dimension: usize,
    pub points: Vec<Point>,
    pub id_to_index: HashMap<Uuid, usize>,
}

pub struct Collection {
    pub embedding_type: EmbeddingType,
    pub dimension: usize,
    pub points: Vec<Point>,
    pub id_to_index: HashMap<Uuid, usize>,
    pub hnsw: Hnsw<'static, f32, DistCosine>,
}

impl Collection {
    pub fn new(embedding_type: EmbeddingType) -> Self {
        let dimension = embedding_type.dimension();
        let hnsw = Hnsw::new(16, 10_000, dimension, 200, DistCosine);
        Self {
            embedding_type,
            dimension,
            points: Vec::new(),
            id_to_index: HashMap::new(),
            hnsw,
        }
    }

    pub fn upsert(&mut self, point: Point) -> Result<(), VectorDbError> {
        if point.vector.len() != self.dimension {
            return Err(VectorDbError::DimensionMismatch {
                expected: self.dimension,
                actual: point.vector.len(),
            });
        }

        if let Some(&idx) = self.id_to_index.get(&point.id) {
            self.points[idx] = point.clone();
            self.hnsw.insert((&point.vector, idx));
        } else {
            let idx = self.points.len();
            self.points.push(point.clone());
            self.id_to_index.insert(point.id, idx);
            self.hnsw.insert((&point.vector, idx));
        }
        Ok(())
    }

    pub fn search(&self, query: &[f32], limit: usize) -> Vec<Point> {
        self.hnsw
            .search(query, limit, 200)
            .into_iter()
            .map(|Neighbour { d_id, .. }| self.points[d_id].clone())
            .collect()
    }

    pub fn from_data(data: CollectionData) -> Self {
        let hnsw = Hnsw::new(16, 10_000, data.dimension, 200, DistCosine);

        for (idx, point) in data.points.iter().enumerate() {
            hnsw.insert((&point.vector, idx));
        }

        Self {
            embedding_type: data.embedding_type,
            dimension: data.dimension,
            points: data.points,
            id_to_index: data.id_to_index,
            hnsw,
        }
    }
}
