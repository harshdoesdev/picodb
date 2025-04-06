use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum EfSearch {
    Fast,
    Balanced,
    Accurate,
}

impl EfSearch {
    pub fn value(self) -> usize {
        match self {
            Self::Fast => 50,
            Self::Balanced => 200,
            Self::Accurate => 500,
        }
    }
}

pub type MetadataFilter = HashMap<String, String>;
