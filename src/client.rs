use pikodb::{
    collection::Collection,
    embedding::Point,
    error::{PersistenceError, VectorDbError},
    index::IndexConfig,
    persistence_adapter::{PersistedState, Persistence},
    search::{EfSearch, MetadataFilter},
    state::{CollectionData, CollectionState},
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
                    .map(|(name, coll_state)| (name, Collection::from_state(coll_state)))
                    .collect();
            }
        }
        client
    }

    pub fn create_collection(
        &mut self,
        name: &str,
        index_config: IndexConfig,
    ) -> Result<(), VectorDbError> {
        if self.collections.contains_key(name) {
            return Ok(());
        }
        self.collections
            .insert(name.to_string(), Collection::with_index_config(index_config));
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

    pub fn query_with_filter(
        &self,
        collection_name: &str,
        query_vector: &[f32],
        limit: usize,
        ef: EfSearch,
        filters: Vec<MetadataFilter>,
    ) -> Result<Vec<Point>, VectorDbError> {
        let collection = self.get_collection(collection_name)?;
        Ok(collection.search_with_filter(query_vector, limit, ef, filters))
    }

    pub fn query(
        &self,
        collection_name: &str,
        query_vector: &[f32],
        limit: usize,
        ef: EfSearch,
    ) -> Result<Vec<Point>, VectorDbError> {
        self.query_with_filter(collection_name, query_vector, limit, ef, vec![])
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
                        CollectionState {
                            data: CollectionData {
                                points: coll.points.clone(),
                                id_to_index: coll.id_to_index.clone(),
                            },
                            index_config: coll.index_config,
                        },
                    )
                })
                .collect(),
        };
        adapter.save(&state)?;
        Ok(())
    }
}
