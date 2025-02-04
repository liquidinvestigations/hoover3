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
    Object(Vec<(String, Box<DatabaseColumnType>)>),
    List(Box<DatabaseColumnType>),
    Other(String),
}

impl std::fmt::Display for DatabaseColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object(o) => write!(
                f,
                "Object {{\n  {}\n}}",
                o.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(",\n  ")
            ),
            Self::List(l) => write!(f, "List[{}]", l),
            Self::String => write!(f, "String"),
            Self::Int8 => write!(f, "Int8"),
            Self::Int16 => write!(f, "Int16"),
            Self::Int32 => write!(f, "Int32"),
            Self::Int64 => write!(f, "Int64"),
            Self::Float => write!(f, "Float"),
            Self::Double => write!(f, "Double"),
            Self::Boolean => write!(f, "Boolean"),
            Self::Timestamp => write!(f, "Timestamp"),
            Self::Other(o) => write!(f, "Other: {}", o),
        }
    }
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

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum DatabaseValue {
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    Timestamp(chrono::DateTime<chrono::Utc>),
    Other(String),
    List(Vec<DatabaseValue>),
    Object(BTreeMap<String, Option<DatabaseValue>>),
}

impl std::fmt::Display for DatabaseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Int8(i) => write!(f, "{}", i),
            Self::Int16(i) => write!(f, "{}", i),
            Self::Int32(i) => write!(f, "{}", i),
            Self::Int64(i) => write!(f, "{}", i),
            Self::Float(ff) => write!(f, "{}", ff),
            Self::Double(d) => write!(f, "{}", d),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Timestamp(t) => write!(f, "{}", t),
            Self::Other(o) => write!(f, "Other: {}", o),
            Self::List(l) => write!(f, "{:?}", l),
            Self::Object(o) => write!(
                f,
                "{}",
                o.iter()
                    .map(|(k, v)| if let Some(v) = v {
                        format!("{}: {}", k, v)
                    } else {
                        format!("{}: null", k)
                    })
                    .collect::<Vec<String>>()
                    .join(",\n")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct DynamicQueryResponse {
    pub query: String,
    pub db_type: DatabaseType,
    pub elapsed_seconds: f64,
    pub result: Result<DynamicQueryResult, String>,
    pub next_page: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct DynamicQueryResult {
    pub columns: Vec<(String, DatabaseColumnType)>,
    pub rows: Vec<Vec<Option<DatabaseValue>>>,
}

impl DynamicQueryResult {
    pub fn from_single_string(result: String) -> anyhow::Result<Self> {
        let result_columns = vec![("_".to_string(), DatabaseColumnType::String)];
        let result_rows = vec![vec![Some(DatabaseValue::String(result))]];
        Ok(DynamicQueryResult {
            columns: result_columns,
            rows: result_rows,
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum DatabaseType {
    Scylla,
    Nebula,
    Meilisearch,
}
