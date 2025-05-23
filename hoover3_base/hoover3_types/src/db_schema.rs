//! Types and structures related to database schemas - Scylla, Nebula, Meilisearch, tables, columns, values.

use std::collections::BTreeMap;

use crate::identifier::{CollectionId, DatabaseIdentifier};

/// Represents the complete schema for a collection across different database types.
/// This type is supposed to be obtained by querying the database for the schema.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct CollectionSchemaDynamic {
    /// Unique identifier for the collection
    pub collection_id: CollectionId,
    /// Schema information for Scylla database
    pub scylla: ScyllaDatabaseSchema,
    /// Schema information for Meilisearch database
    pub meilisearch: MeilisearchDatabaseSchema,
    /// Schema info for graph db
    pub graph: GraphEdgeSchemaDynamic,
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

    /// Field definition for the column, if it is part of a model
    pub field_definition: Option<ModelFieldDefinition>,
}

/// Represents an edge type in a graph database
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgeId(
    /// Name/identifier of the edge type
    pub DatabaseIdentifier,
);

impl std::fmt::Display for GraphEdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the possible data types for database columns
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
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
    /// Object type with named fields and their types
    Object(BTreeMap<String, Box<DatabaseColumnType>>),
    /// List/array of values of a specific type
    List(Box<DatabaseColumnType>),
    /// Custom or unknown data type
    Other(String),
    /// Graph Vertex - contains multiple tags, each with multiple fields
    GraphVertex(BTreeMap<String, BTreeMap<String, Box<DatabaseColumnType>>>),
    /// Graph Edge
    GraphEdge,
    /// Type is not one of the above, so macros do not expand it
    ///  - to be converted to concrete type at runtime
    UnspecifiedType,
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
            Self::GraphVertex(g) => write!(f, "GraphVertex: {:#?}", g),
            Self::GraphEdge => write!(f, "GraphEdge"),
            Self::UnspecifiedType => write!(f, "UnspecifiedType"),
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
            Self::String => "Text".to_string(),
            Self::Int8 => "TinyInt".to_string(),
            Self::Int16 => "SmallInt".to_string(),
            Self::Int32 => "Int".to_string(),
            Self::Int64 => "BigInt".to_string(),
            Self::Float => "Float".to_string(),
            Self::Double => "Double".to_string(),
            Self::Boolean => "Boolean".to_string(),
            Self::Timestamp => "Timestamp".to_string(),
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

/// Represents a timestamp value
pub type Timestamp = chrono::DateTime<chrono::Utc>;

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
    Timestamp(Timestamp),
    /// Custom or unknown value type
    Other(String),
    /// List/array of database values
    List(Vec<DatabaseValue>),
    /// Complex object with optional field values
    Object(BTreeMap<String, Option<DatabaseValue>>),
    /// Graph Vertex - one ID + one or more tags
    GraphVertex {
        /// Vertex ID - short string
        id: String,
        /// Vertex tags - map of tag name to map of field name to value
        tags: BTreeMap<String, BTreeMap<String, Option<DatabaseValue>>>,
    },
    /// Graph Edge
    GraphEdge {
        /// Edge Type
        edge_type: String,
        /// Source vertex ID
        source_vertex: String,
        /// Target vertex ID
        target_vertex: String,
        /// Ranking, as in nebula feature "a" -> "b" @ 0 {}
        ranking: i64,
        // TODO: edge properties.
    },
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
            Self::GraphVertex { id, tags } => {
                write!(f, "GraphVertex {{ id: {}, tags: {:#?} }}", id, tags)
            }
            Self::GraphEdge { .. } => {
                write!(f, "{:#?}", self)
            }
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
    /// Serialized dataset size, in bytes
    pub result_serialized_size_bytes: u64,
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
    /// Meilisearch search engine
    Meilisearch,
}

/// Represents the definition of a model - the result of parsing a struct tagged with
///  #[model]
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct ModelDefinition {
    /// The name of the table in the database
    pub table_name: String,
    /// The name of the model in the source code
    pub model_name: String,
    /// The fields of the model in their struct order
    pub fields: Vec<ModelFieldDefinition>,
    /// Docstring of model
    pub docstring: String,
    /// Rust code of the Charybdis definition
    pub charybdis_code: String,
}

/// Represents the definition of a UDT - the result of parsing a struct tagged with
///  #[udt_model]
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct UdtModelDefinition {
    /// The name of the table in the database
    pub udt_name: String,
    /// The name of the model in source code
    pub model_name: String,
    /// The fields of the model in their struct order
    pub fields: Vec<ModelFieldDefinition>,
    /// Docstring of model
    pub docstring: String,
    /// Rust code of the Charybdis definition
    pub charybdis_code: String,
}

/// Represents the definition of a field in a model (a struct tagged with #[model])
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct ModelFieldDefinition {
    /// The name of the field in the struct
    pub name: String,
    /// The type of the field in the struct
    pub field_type: DatabaseColumnType,
    /// Whether the field is part of the CQL primary key
    pub partition_key: bool,
    /// Whether the field is part of the CQL clustering key
    pub clustering_key: bool,
    /// Whether the field is stored in the search index
    pub search_store: bool,
    /// Whether the field is indexed in the search index
    pub search_index: bool,
    /// Whether the field is used for search faceting
    pub search_facet: bool,
    /// Docstring of field
    pub docstring: String,
    /// Nullable - field is of type Option<T>
    pub nullable: bool,
    /// Field type as a string - the original in rust code. Useful when field_type is unspecified in the first pass.
    pub field_type_original: String,
}

/// Dynamic version of GraphEdgeType - used for runtime queries.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgeTypeDynamic {
    /// The name of the edge type
    pub edge_type: DatabaseIdentifier,
    /// The source node type
    pub source_type: DatabaseIdentifier,
    /// The target node type
    pub target_type: DatabaseIdentifier,
    /// Implementation of the edge store
    pub edge_store_type: EdgeStoreImplementation,
}

/// Implementation of the edge store
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum EdgeStoreImplementation {
    /// Scylla Graph Tables models
    Stored = 1,
    /// Scylla Parent/Child Implicit where
    /// the partition key of the child is the primary key of the parent
    Implicit = 2,
}

/// Schema for graph edges - contains all edge types, and for each edge type, the
/// source and target types (table names).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct GraphEdgeSchemaDynamic {
    /// Map of edge type names to their definitions
    pub edges_by_types: BTreeMap<GraphEdgeId, GraphEdgeTypeDynamic>,
    /// Map of source node types to their edge types
    pub edges_by_source: BTreeMap<DatabaseIdentifier, Vec<GraphEdgeTypeDynamic>>,
    /// Map of target node types to their edge types
    pub edges_by_target: BTreeMap<DatabaseIdentifier, Vec<GraphEdgeTypeDynamic>>,
}
