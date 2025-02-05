//! Types and structures related to database schemas - Scylla, Nebula, Meilisearch, tables, columns, values.

use std::collections::BTreeMap;

use crate::identifier::{CollectionId, DatabaseIdentifier};

/// Represents the complete schema for a collection across different database types
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct CollectionSchema {
    /// Unique identifier for the collection
    pub collection_id: CollectionId,
    /// Schema information for Scylla database
    pub scylla: ScyllaDatabaseSchema,
    /// Schema information for Nebula database
    pub nebula: NebulaDatabaseSchema,
    /// Schema information for Meilisearch database
    pub meilisearch: MeilisearchDatabaseSchema,
}

/// Schema information specific to Meilisearch database
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct MeilisearchDatabaseSchema {
    /// Total number of documents in the collection
    pub doc_count: i64,
    /// Map of field names to their occurrence count in the documents
    pub fields: BTreeMap<String, i64>,
}

/// Schema information specific to Scylla database
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct ScyllaDatabaseSchema {
    /// Map of table identifiers to their corresponding table definitions
    pub tables: BTreeMap<DatabaseIdentifier, DatabaseTable>,
}

/// Schema information specific to Nebula graph database
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct NebulaDatabaseSchema {
    /// Map of vertex tag identifiers to their corresponding table definitions
    pub tags: BTreeMap<DatabaseIdentifier, DatabaseTable>,
    /// List of edge types defined in the graph
    pub edges: Vec<GraphEdgeType>,
}

/// Represents a database table structure
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DatabaseTable {
    /// Name/identifier of the table
    pub name: DatabaseIdentifier,
    /// List of columns defined in the table
    pub columns: Vec<DatabaseColumn>,
}

/// Represents a column in a database table
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DatabaseColumn {
    /// Name/identifier of the column
    pub name: DatabaseIdentifier,
    /// Data type of the column
    pub _type: DatabaseColumnType,
    /// Indicates if this column is part of the primary key
    pub primary: bool,
}

/// Represents an edge type in a graph database
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgeType {
    /// Name/identifier of the edge type
    pub name: DatabaseIdentifier,
}

/// Represents the possible data types for database columns
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum DatabaseColumnType {
    /// Text/string data type
    String,
    /// 8-bit integer
    Int8,
    /// 16-bit integer
    Int16,
    /// 32-bit integer
    Int32,
    /// 64-bit integer
    Int64,
    /// 32-bit floating point
    Float,
    /// 64-bit floating point
    Double,
    /// Boolean value
    Boolean,
    /// Timestamp value
    Timestamp,
    /// Complex object type with named fields and their types
    Object(Vec<(String, Box<DatabaseColumnType>)>),
    /// List/array of values of a specific type
    List(Box<DatabaseColumnType>),
    /// Custom or unknown data type
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
    /// Converts a Scylla database type string into its corresponding DatabaseColumnType
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

    /// Converts this DatabaseColumnType into its corresponding Scylla type string representation
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

    /// Converts a Nebula database type string into its corresponding DatabaseColumnType
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

    /// Converts this DatabaseColumnType into its corresponding Nebula type string representation
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

/// Represents actual values stored in the database
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum DatabaseValue {
    /// Text/string value
    String(String),
    /// 8-bit integer value
    Int8(i8),
    /// 16-bit integer value
    Int16(i16),
    /// 32-bit integer value
    Int32(i32),
    /// 64-bit integer value
    Int64(i64),
    /// 32-bit floating point value
    Float(f32),
    /// 64-bit floating point value
    Double(f64),
    /// Boolean value
    Boolean(bool),
    /// UTC timestamp value
    Timestamp(chrono::DateTime<chrono::Utc>),
    /// Custom or unknown value type
    Other(String),
    /// List/array of database values
    List(Vec<DatabaseValue>),
    /// Complex object with optional field values
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

/// Response structure for dynamic database queries
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct DynamicQueryResponse {
    /// The executed query string
    pub query: String,
    /// Type of database that executed the query
    pub db_type: DatabaseServiceType,
    /// Query execution time in seconds
    pub elapsed_seconds: f64,
    /// Query result or error message
    pub result: Result<DynamicQueryResult, String>,
}

/// Contains the results of a dynamic database query
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct DynamicQueryResult {
    /// List of column names and their types
    pub columns: Vec<(String, DatabaseColumnType)>,
    /// Rows of data, where each value is optional
    pub rows: Vec<Vec<Option<DatabaseValue>>>,
    /// Optional pagination token for next page of results
    pub next_page: Option<Vec<u8>>,
}

impl DynamicQueryResult {
    /// Creates a new DynamicQueryResult containing a single string value as its only row
    pub fn from_single_string(result: String) -> anyhow::Result<Self> {
        let result_columns = vec![("_".to_string(), DatabaseColumnType::String)];
        let result_rows = vec![vec![Some(DatabaseValue::String(result))]];
        Ok(DynamicQueryResult {
            columns: result_columns,
            rows: result_rows,
            next_page: None,
        })
    }
}

/// Represents the different types of databases supported
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum DatabaseServiceType {
    /// ScyllaDB (Cassandra-compatible database)
    Scylla,
    /// Nebula Graph database
    Nebula,
    /// Meilisearch search engine
    Meilisearch,
}
