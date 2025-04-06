use hnsw_rs::prelude::*;
use pikodb::{
    constants::DEFAULT_OVERFETCH_FACTOR,
    embedding::Point,
    error::VectorDbError,
    index::IndexConfig,
    search::{EfSearch, MetadataFilter},
    state::CollectionState,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Collection {
    pub points: Vec<Point>,
    pub id_to_index: HashMap<Uuid, usize>,
    pub hnsw: Hnsw<'static, f32, DistCosine>,
    pub index_config: IndexConfig,
}

impl Collection {
    pub fn with_index_config(index_config: IndexConfig) -> Self {
        let hnsw = Hnsw::new(
            16,
            10_000,
            index_config.embedding_type.dimension(),
            index_config.build_quality.value(),
            DistCosine,
        );
        Self {
            points: Vec::new(),
            id_to_index: HashMap::new(),
            hnsw,
            index_config,
        }
    }

    pub fn upsert(&mut self, point: Point) -> Result<(), VectorDbError> {
        let dimension = self.index_config.embedding_type.dimension();

        if point.vector.len() != dimension {
            return Err(VectorDbError::DimensionMismatch {
                expected: dimension,
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

    pub fn search_with_filter(
        &self,
        query: &[f32],
        limit: usize,
        ef: EfSearch,
        filters: Vec<MetadataFilter>,
    ) -> Vec<Point> {
        let fetch_limit = if filters.is_empty() {
            limit
        } else {
            limit * DEFAULT_OVERFETCH_FACTOR
        };

        let candidates = self.hnsw.search(query, fetch_limit, ef.value());

        // TODO: Add support for: lt, gt, full-text/fuzzy search filters
        let filtered: Vec<_> = candidates
            .into_iter()
            .map(|Neighbour { d_id, .. }| &self.points[d_id])
            .filter(|point| {
                if filters.is_empty() {
                    true
                } else {
                    filters
                        .iter()
                        .any(|f| f.iter().all(|(k, v)| point.metadata.get(k) == Some(v)))
                }
            })
            .cloned()
            .collect();

        filtered.into_iter().take(limit).collect()
    }

    pub fn search(&self, query: &[f32], limit: usize, ef: EfSearch) -> Vec<Point> {
        self.search_with_filter(query, limit, ef, vec![])
    }

    pub fn from_state(state: CollectionState) -> Self {
        let data = state.data;
        let index_config = state.index_config;
        let hnsw = Hnsw::new(
            16,
            10_000,
            index_config.embedding_type.dimension(),
            index_config.build_quality.value(),
            DistCosine,
        );
        let collection = Self {
            points: data.points,
            id_to_index: data.id_to_index,
            hnsw,
            index_config,
        };
        for (idx, point) in collection.points.iter().enumerate() {
            collection.hnsw.insert((&point.vector, idx));
        }
        collection
    }
}
