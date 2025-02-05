//! This module implements the `impl_model_callbacks` macro, which is used to add Charybdis callbacks to model structs.
//! These callbacks are used to insert/update/delete rows in the secondary databases, Nebula and Meilisearch.

use super::_nebula_edges::GraphEdgeIdentifier;
use crate::db_management::meilisearch_wait_for_task;
use crate::db_management::nebula_execute_retry;
use crate::db_management::DatabaseSpaceManager;
use crate::db_management::MeilisearchDatabaseHandle;
use crate::migrate::nebula_get_schema;
use charybdis::model::BaseModel;
use hoover3_types::db_schema::GraphEdgeType;
use hoover3_types::db_schema::NebulaDatabaseSchema;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;

fn nebula_sql_insert_vertex(
    table_id: &DatabaseIdentifier,
    schema: NebulaDatabaseSchema,
    data: Vec<(String, serde_json::Value)>,
) -> anyhow::Result<String> {
    use anyhow::Context;

    let schema_table = schema
        .tags
        .get(table_id)
        .context("table not found in schema")?;
    let column_defs = schema_table
        .columns
        .iter()
        .map(|c| format!("`{}`", c.name))
        .collect::<Vec<String>>()
        .join(", ");

    let column_values = data
        .into_iter()
        .map(|(nebula_pk, data_json)| {
            let column_values = schema_table
                .columns
                .iter()
                .map(|c| data_json[c.name.to_string()].to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("\"{nebula_pk}\":({column_values})")
        })
        .collect::<Vec<String>>()
        .join(", ");

    let query = format!(
        "
        INSERT VERTEX `{table_id}` ({column_defs})
        VALUES {column_values};
        "
    );
    Ok(query)
}

#[test]
fn test_nebula_sql_insert_vertex() {
    use hoover3_types::db_schema::DatabaseColumnType;
    use hoover3_types::db_schema::{DatabaseColumn, DatabaseTable};

    let mut schema = NebulaDatabaseSchema {
        tags: std::collections::BTreeMap::new(),
        edges: vec![],
    };
    let table_id = DatabaseIdentifier::new("test_nebula_sql_insert_vertex").unwrap();
    schema.tags.insert(
        table_id.clone(),
        DatabaseTable {
            name: table_id.clone(),
            columns: vec![
                DatabaseColumn {
                    name: DatabaseIdentifier::new("name").unwrap(),
                    _type: DatabaseColumnType::String,
                    primary: true,
                },
                DatabaseColumn {
                    name: DatabaseIdentifier::new("age").unwrap(),
                    _type: DatabaseColumnType::Int64,
                    primary: true,
                },
            ],
        },
    );
    let data = vec![(
        "1".to_string(),
        serde_json::json!({"name": "John", "age": 30}),
    )];
    let query = nebula_sql_insert_vertex(&table_id, schema, data)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    assert_eq!(
        query.trim(),
        "INSERT VERTEX `test_nebula_sql_insert_vertex` (`name`, `age`) VALUES \"1\":(\"John\", 30);"
    );
}

fn nebula_sql_insert_edge(
    edge: &GraphEdgeType,
    data: Vec<(String, String)>,
) -> anyhow::Result<String> {
    let edge_name = &edge.name;
    let edge_rank: u32 = 0;
    let query = data
        .into_iter()
        .map(|(from, to)| format!("\"{from}\"->\"{to}\"@{edge_rank}:()"))
        .collect::<Vec<String>>()
        .join(",\n");
    Ok(format!(
        "
    INSERT EDGE `{edge_name}` () VALUES {query};
    "
    ))
}

#[tokio::test]
async fn test_nebula_sql_insert_edge() {
    let edge = GraphEdgeType {
        name: DatabaseIdentifier::new("test_edge").unwrap(),
    };
    let data = vec![("1".to_string(), "2".to_string())];
    let query = nebula_sql_insert_edge(&edge, data).unwrap();
    assert_eq!(
        query.trim(),
        "INSERT EDGE `test_edge` () VALUES \"1\"->\"2\"@0:();"
    );

    let data = vec![
        ("1".to_string(), "2".to_string()),
        ("3".to_string(), "4".to_string()),
    ];
    let query = nebula_sql_insert_edge(&edge, data)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    assert_eq!(
        query.trim(),
        "INSERT EDGE `test_edge` () VALUES \"1\"->\"2\"@0:(), \"3\"->\"4\"@0:();"
    );
}

/// Batch insert edges into a Nebula database.
pub struct InsertEdgeBatch<T1: BaseModel, T2: BaseModel> {
    edge: GraphEdgeType,
    data: Vec<(String, String)>,
    _phantom: std::marker::PhantomData<(T1, T2)>,
}

impl<T1: BaseModel, T2: BaseModel> InsertEdgeBatch<T1, T2>
where
    T1: BaseModel,
    T2: BaseModel,
    <T1 as BaseModel>::PrimaryKey: serde::Serialize + 'static,
    <T2 as BaseModel>::PrimaryKey: serde::Serialize + 'static,
    <T1 as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
    <T2 as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
{
    /// Create a new empty batch.
    pub fn new(edge: impl GraphEdgeIdentifier) -> Self {
        Self {
            edge: edge.to_owned(),
            data: vec![],
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a new edge to the batch.
    pub fn push(&mut self, from: &T1, to: &T2) {
        self.data
            .push((row_pk_hash::<T1>(from), row_pk_hash::<T2>(to)));
    }

    /// Execute the batch insert, consuming the batch.
    pub async fn execute(self, db_extra: &DatabaseExtraCallbacks) -> anyhow::Result<()> {
        if self.data.is_empty() {
            return Ok(());
        }
        let query = nebula_sql_insert_edge(&self.edge, self.data)?;
        nebula_execute_retry::<()>(&db_extra.collection_id, &query).await?;
        Ok(())
    }
}

/// Compute a stable hash of a row's primary key, and concatenate it with table name.
pub fn row_pk_hash<T>(data: &T) -> String
where
    T: BaseModel,
    <T as BaseModel>::PrimaryKey: serde::Serialize,
    <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
    <T as BaseModel>::PrimaryKey: 'static,
{
    use hoover3_types::stable_hash::stable_hash;
    let data = data.primary_key_values();
    let x = stable_hash(&data).expect("can compute stable hash");
    let table_name = T::DB_MODEL_NAME;
    format!("{table_name}_{x}")
}

/// Get a JSON representation of a row for indexing in Meilisearch.
pub fn get_search_index_json<T>(row: &T) -> anyhow::Result<serde_json::Value>
where
    T: BaseModel + serde::Serialize,
    <T as BaseModel>::PrimaryKey: serde::Serialize,
    <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
    <T as BaseModel>::PrimaryKey: 'static,
{
    let table_name = T::DB_MODEL_NAME;
    let row_pk = row_pk_hash::<T>(row);
    let data_json = serde_json::to_value(row)?;
    flatten_index_data(serde_json::json!({
        "id": row_pk,
        "table": table_name,
        table_name: data_json,
    }))
}

fn flatten_index_data(data: serde_json::Value) -> anyhow::Result<serde_json::Value> {
    match data {
        serde_json::Value::Object(obj) => {
            let mut flat_obj = serde_json::Map::new();
            _flatten_object("", obj, &mut flat_obj);
            Ok(serde_json::Value::Object(flat_obj))
        }
        _ => Ok(data),
    }
}

fn _flatten_object(
    prefix: &str,
    obj: serde_json::Map<String, serde_json::Value>,
    output: &mut serde_json::Map<String, serde_json::Value>,
) {
    for (key, value) in obj {
        let new_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{}:{}", prefix, key)
        };

        match value {
            serde_json::Value::Object(nested_obj) => {
                _flatten_object(&new_key, nested_obj, output);
            }
            serde_json::Value::Array(arr) => {
                // Create maps to collect unique values for each field
                let mut field_values: std::collections::BTreeMap<String, Vec<serde_json::Value>> =
                    std::collections::BTreeMap::new();

                // Process each object in the array
                for item in arr {
                    if let serde_json::Value::Object(obj) = item {
                        for (field_key, field_value) in obj {
                            let full_key = format!("{}:{}", new_key, field_key);
                            field_values.entry(full_key).or_default().push(field_value);
                        }
                    }
                }

                // Add collected values to output
                for (field_key, values) in field_values {
                    output.insert(field_key, serde_json::Value::Array(values));
                }
            }
            _ => {
                output.insert(new_key, value);
            }
        }
    }
}

#[test]
fn test_flatten_index_data() {
    macro_rules! _test_flatten {
        ($a:tt, $b:tt) => {
            let a = serde_json::json!($a);
            let b = serde_json::json!($b);
            let flat = flatten_index_data(a).unwrap();
            assert_eq!(flat, b);
        };
    }
    _test_flatten!(
        {
            "a": {
                "b": {
                    "c": "d"
                }
            }
        },
        {
            "a:b:c": "d"
        }
    );
    _test_flatten!(
        {
            "id": 0,
            "patient_name": {
              "forename": "Imogen",
              "surname": "Temult"
            }
        },
        {  "id": 0,
            "patient_name:forename": "Imogen",
            "patient_name:surname": "Temult"
        }
    );
    _test_flatten!(
        {
            "id": 0,
            "patient_name": "Imogen Temult",
            "appointments": [
              {
                "date": "2022-01-01",
                "doctor": "Jester Lavorre",
                "ward": "psychiatry"
              },
              {
                "date": "2019-01-01",
                "doctor": "Dorian Storm"
              }
            ]
        },
        {
            "id": 0,
            "patient_name": "Imogen Temult",
            "appointments:date": [
              "2022-01-01",
              "2019-01-01"
            ],
            "appointments:doctor": [
              "Jester Lavorre",
              "Dorian Storm"
            ],
            "appointments:ward": [
              "psychiatry"
            ]
        }
    );
}

/// Extra runtime information for running database insert/update/delete operations across multiple databases.
/// This struct consists of various database handles and schema information, to be used in row callbacks.
pub struct DatabaseExtraCallbacks {
    /// Unique identifier for the collection
    pub collection_id: CollectionId,
    /// Schema of the Nebula database
    pub nebula_schema: NebulaDatabaseSchema,
    /// Handle to the Meilisearch database
    pub search_index: std::sync::Arc<
        <meilisearch_sdk::client::Client as DatabaseSpaceManager>::CollectionSessionType,
    >,
}

impl DatabaseExtraCallbacks {
    /// Create a new `DatabaseExtraCallbacks` instance by opening sessiosn and fetching schemas..
    pub async fn new(c: &CollectionId) -> anyhow::Result<Self> {
        let nebula_schema = nebula_get_schema(c).await?;
        let search_client = MeilisearchDatabaseHandle::collection_session(c).await?;
        Ok(Self {
            collection_id: c.clone(),
            nebula_schema,
            search_index: search_client,
        })
    }

    /// Insert a batch of rows into the secondary databases, Nebula and Meilisearch.
    pub async fn insert<T>(&self, data: &[T]) -> anyhow::Result<()>
    where
        T: BaseModel + serde::Serialize + Send,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static + Send,
    {
        if data.is_empty() {
            return Ok(());
        }
        let table_id = DatabaseIdentifier::new(T::DB_MODEL_NAME)?;
        let mut nebula_data = vec![];
        let mut search_data = vec![];
        for d in data.iter() {
            let row_pk = row_pk_hash::<T>(d);
            let data_json = serde_json::to_value(d)?;
            nebula_data.push((row_pk.clone(), data_json.clone()));

            search_data.push(get_search_index_json(d)?);
        }
        use tokio::time::Duration;

        let _search_result = tokio::time::timeout(
            Duration::from_secs(30),
            self.search_index.add_documents(&search_data, Some("id")),
        )
        .await??;

        let nebula_insert_query =
            nebula_sql_insert_vertex(&table_id, self.nebula_schema.clone(), nebula_data)?;
        nebula_execute_retry::<()>(&self.collection_id, &nebula_insert_query).await?;

        // takes too much time
        // meilisearch_wait_for_task(_search_result).await?;

        Ok(())
    }

    /// Delete a batch of rows from the secondary databases, Nebula and Meilisearch.
    pub async fn delete<T>(&self, data: &[T]) -> anyhow::Result<()>
    where
        T: BaseModel + Send,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static + Send,
    {
        if data.is_empty() {
            return Ok(());
        }
        let pks = data
            .iter()
            .map(|d| row_pk_hash::<T>(d))
            .collect::<Vec<String>>();

        let _search_result = self.search_index.delete_documents(&pks).await?;

        let nebula_pks = pks
            .into_iter()
            .map(|pk| format!("\"{pk}\""))
            .collect::<Vec<String>>()
            .join(",");
        let nebula_delete_query = format!(
            "
            DELETE VERTEX {nebula_pks} WITH EDGE;
            "
        );
        nebula_execute_retry::<()>(&self.collection_id, &nebula_delete_query).await?;

        // takes too much time
        meilisearch_wait_for_task(_search_result).await?;

        Ok(())
    }
}

/// Macro to implement Charybdis callbacks for a model struct.
#[macro_export]
macro_rules! impl_model_callbacks {
    ($name:ident) => {
        impl charybdis::callbacks::Callbacks for $name {
            type Error = anyhow::Error;
            type Extension = $crate::models::collection::DatabaseExtraCallbacks;

            async fn after_insert(
                &mut self,
                _session: &charybdis::scylla::CachingSession,
                extension: &$crate::models::collection::DatabaseExtraCallbacks,
            ) -> anyhow::Result<()> {
                extension.insert(&[self.clone()]).await
            }

            async fn after_delete(
                &mut self,
                _session: &charybdis::scylla::CachingSession,
                extension: &$crate::models::collection::DatabaseExtraCallbacks,
            ) -> anyhow::Result<()> {
                extension.delete(&[self.clone()]).await
            }
        }
    };
}
pub use impl_model_callbacks;
