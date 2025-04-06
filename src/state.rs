use pikodb::{embedding::Point, index::IndexConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CollectionData {
    pub dimension: usize,
    pub points: Vec<Point>,
    pub id_to_index: HashMap<Uuid, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionState {
    pub data: CollectionData,
    pub index_config: IndexConfig,
}
