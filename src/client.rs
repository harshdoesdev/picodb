use pikodb::{
    collection::{Collection, CollectionData},
    embedding::{EmbeddingType, Point},
    error::{PersistenceError, VectorDbError},
    persistence_adapter::{PersistedState, Persistence},
};
use std::collections::HashMap;

pub struct ClientConfig {
    pub persistence_adapter: Option<Persistence>,
}

pub struct Client {
    collections: HashMap<String, Collection>,
    persistence_adapter: Option<Persistence>,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        let mut client = Self {
            collections: HashMap::new(),
            persistence_adapter: config.persistence_adapter,
        };

        if let Some(adapter) = &client.persistence_adapter {
            if let Ok(state) = adapter.load() {
                client.collections = state
                    .collections
                    .into_iter()
                    .map(|(name, data)| (name, Collection::from_data(data)))
                    .collect();
            }
        }
        client
    }

    pub fn create_collection(
        &mut self,
        name: &str,
        embedding_type: EmbeddingType,
    ) -> Result<(), VectorDbError> {
        if self.collections.contains_key(name) {
            return Ok(());
        }
        self.collections
            .insert(name.to_string(), Collection::new(embedding_type));
        self.persist()?;
        Ok(())
    }

    pub fn get_collection(&self, name: &str) -> Result<&Collection, VectorDbError> {
        self.collections
            .get(name)
            .ok_or(VectorDbError::CollectionNotFound {
                name: name.to_string(),
            })
    }

    pub fn get_or_create_collection(
        &mut self,
        name: &str,
        embedding_type: EmbeddingType,
    ) -> &mut Collection {
        self.collections
            .entry(name.to_string())
            .or_insert_with(|| Collection::new(embedding_type))
    }

    pub fn upsert_points(
        &mut self,
        collection_name: &str,
        points: Vec<Point>,
    ) -> Result<(), VectorDbError> {
        let collection =
            self.collections
                .get_mut(collection_name)
                .ok_or(VectorDbError::CollectionNotFound {
                    name: collection_name.to_string(),
                })?;

        for point in points {
            collection.upsert(point)?;
        }
        self.persist()?;
        Ok(())
    }

    pub fn query(
        &self,
        collection_name: &str,
        query_vector: &[f32],
        limit: usize,
    ) -> Result<Vec<Point>, VectorDbError> {
        let collection = self.get_collection(collection_name)?;
        Ok(collection.search(query_vector, limit))
    }

    fn persist(&self) -> Result<(), VectorDbError> {
        let Some(adapter) = &self.persistence_adapter else {
            return Err(VectorDbError::PersistenceError(
                PersistenceError::NotConfigured,
            ));
        };

        let state = PersistedState {
            collections: self
                .collections
                .iter()
                .map(|(name, coll)| {
                    (
                        name.clone(),
                        CollectionData {
                            embedding_type: coll.embedding_type.clone(),
                            dimension: coll.dimension,
                            points: coll.points.clone(),
                            id_to_index: coll.id_to_index.clone(),
                        },
                    )
                })
                .collect(),
        };
        adapter.save(&state)?;
        Ok(())
    }
}
