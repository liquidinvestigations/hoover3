use std::collections::BTreeMap;

use crate::identifier::{CollectionId, DatabaseIdentifier};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct CollectionSchema {
    pub collection_id: CollectionId,
    pub scylla: ScyllaDatabaseSchema,
    pub nebula: NebulaDatabaseSchema,
    pub meilisearch: MeilisearchDatabaseSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct MeilisearchDatabaseSchema {
    pub doc_count: i64,
    pub fields: BTreeMap<String, i64>,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct ScyllaDatabaseSchema {
    pub tables: BTreeMap<DatabaseIdentifier, DatabaseTable>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct NebulaDatabaseSchema {
    pub tags: BTreeMap<DatabaseIdentifier, DatabaseTable>,
    pub edges: Vec<GraphEdgeType>,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DatabaseTable {
    pub name: DatabaseIdentifier,
    pub columns: Vec<DatabaseColumn>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DatabaseColumn {
    pub name: DatabaseIdentifier,
    pub _type: DatabaseColumnType,
    pub primary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgeType {
    pub name: DatabaseIdentifier,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum DatabaseColumnType {
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    Boolean,
    Timestamp,
    Option(Box<DatabaseColumnType>),
    Other(String),
}

impl DatabaseColumnType {
    pub fn from_scylla_type(s: &str) -> anyhow::Result<Self> {
        Ok(match s.to_uppercase().as_str() {
            "VARCHAR" | "TEXT" => Self::String,
            "TINYINT" => Self::Int8,
            "SMALLINT" => Self::Int16,
            "INT" => Self::Int32,
            "BIGINT" => Self::Int64,
            "FLOAT" => Self::Float,
            "DOUBLE" => Self::Double,
            "BOOLEAN" => Self::Boolean,
            "TIMESTAMP" => Self::Timestamp,
            _x => Self::Other(_x.to_string()),
        })
    }
    pub fn to_scylla_type(&self) -> anyhow::Result<String> {
        Ok(match self {
            Self::String => "TEXT".to_string(),
            Self::Int8 => "TINYINT".to_string(),
            Self::Int16 => "SMALLINT".to_string(),
            Self::Int32 => "INT".to_string(),
            Self::Int64 => "BIGINT".to_string(),
            Self::Float => "FLOAT".to_string(),
            Self::Double => "DOUBLE".to_string(),
            Self::Boolean => "BOOLEAN".to_string(),
            Self::Timestamp => "TIMESTAMP".to_string(),
            _ => anyhow::bail!("incompatible with scylla type: {:?}", self),
        })
    }
    pub fn from_nebula_type(s: &str) -> anyhow::Result<Self> {
        Ok(match s.to_uppercase().as_str() {
            "STRING" => Self::String,
            "INT8" => Self::Int8,
            "INT16" => Self::Int16,
            "INT32" => Self::Int32,
            "INT64" => Self::Int64,
            "FLOAT" => Self::Float,
            "DOUBLE" => Self::Double,
            "BOOLEAN" => Self::Boolean,
            "TIMESTAMP" => Self::Timestamp,
            _ => anyhow::bail!("unknown nebula type: {}", s),
        })
    }
    pub fn to_nebula_type(&self) -> anyhow::Result<String> {
        Ok(match self {
            Self::String => "STRING".to_string(),
            Self::Int8 => "INT8".to_string(),
            Self::Int16 => "INT16".to_string(),
            Self::Int32 => "INT32".to_string(),
            Self::Int64 => "INT64".to_string(),
            Self::Float => "FLOAT".to_string(),
            Self::Double => "DOUBLE".to_string(),
            Self::Boolean => "BOOLEAN".to_string(),
            _ => anyhow::bail!("incompatible with nebula type: {:?}", self),
        })
    }


}
