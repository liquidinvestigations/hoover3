//! This module implements the `impl_model_callbacks` macro, which is used to add Charybdis callbacks to model structs.
//! These callbacks are used to insert/update/delete rows in the secondary databases, Nebula and Meilisearch.


use crate::db_management::meilisearch_wait_for_task;
use crate::db_management::nebula_execute_retry;
use crate::db_management::DatabaseSpaceManager;
use crate::db_management::MeilisearchDatabaseHandle;
use crate::models::collection::graph_add_nodes;
use charybdis::model::BaseModel;

use hoover3_types::db_schema::NebulaDatabaseSchema;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;


/// Compute a stable hash of a row's primary key, and concatenate it with table name.
pub fn row_pk_hash<T>(data: &T::PrimaryKey) -> String
where
    T: BaseModel,
    <T as BaseModel>::PrimaryKey: serde::Serialize,
    <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
    <T as BaseModel>::PrimaryKey: 'static,
{
    use hoover3_types::stable_hash::stable_hash;
    // let data = data.primary_key_values();
    let x = stable_hash(data).expect("can compute stable hash");
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
    let row_pk = row_pk_hash::<T>(&row.primary_key_values());
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
        let nebula_schema = crate::db_management::query_nebula_schema(c).await?;
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
        T: BaseModel + serde::Serialize + Send + Sync + 'static,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static + Send + Sync,
    {
        if data.is_empty() {
            return Ok(());
        }
        let _table_id = DatabaseIdentifier::new(T::DB_MODEL_NAME)?;
        let mut nebula_data = vec![];
        let mut search_data = vec![];
        for d in data.iter() {
            let row_pk_hash = row_pk_hash::<T>(&d.primary_key_values());
            let data_json = serde_json::to_value(d)?;
            nebula_data.push((row_pk_hash.clone(), data_json.clone()));

            search_data.push(get_search_index_json(d)?);
        }
        use tokio::time::Duration;

        let _search_result = tokio::time::timeout(
            Duration::from_secs(30),
            self.search_index.add_documents(&search_data, Some("id")),
        )
        .await??;

        // let nebula_insert_query =
        // nebula_sql_insert_vertex(&table_id, self.nebula_schema.clone(), nebula_data)?;
        // nebula_execute_retry::<()>(&self.collection_id, &nebula_insert_query).await?;
        graph_add_nodes(&self.collection_id, data).await?;

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
            .map(|d| row_pk_hash::<T>(&d.primary_key_values()))
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
        impl ::charybdis::callbacks::Callbacks for $name {
            /// Error type for the callbacks - we always use anyhow.
            type Error = ::anyhow::Error;
            /// Extension type for the callbacks - see [DatabaseExtraCallbacks].
            type Extension = $crate::models::collection::DatabaseExtraCallbacks;

            /// Callback calls the `insert` method on the `DatabaseExtraCallbacks` instance.
            async fn after_insert(
                &mut self,
                _session: &::charybdis::scylla::CachingSession,
                extension: &$crate::models::collection::DatabaseExtraCallbacks,
            ) -> ::anyhow::Result<()> {
                extension.insert(&[self.clone()]).await
            }

            /// Callback calls the `delete` method on the `DatabaseExtraCallbacks` instance.
            async fn after_delete(
                &mut self,
                _session: &::charybdis::scylla::CachingSession,
                extension: &$crate::models::collection::DatabaseExtraCallbacks,
            ) -> ::anyhow::Result<()> {
                extension.delete(&[self.clone()]).await
            }
        }

        impl $name {
            /// Compute a stable hash of a row's primary key, and concatenate it with table name.
            pub fn row_pk_hash(&self) -> String {
                use ::charybdis::model::BaseModel;
                $crate::models::collection::row_pk_hash::<$name>(&self.primary_key_values())
            }

            /// Get a JSON representation of a row's primary key.
            pub fn row_pk_json(&self) -> ::anyhow::Result<::serde_json::Value> {
                use ::charybdis::model::BaseModel;
                Ok(::serde_json::to_value(&self.primary_key_values())?)
            }
        }
    };
}
/// Re-export:
pub use impl_model_callbacks;
