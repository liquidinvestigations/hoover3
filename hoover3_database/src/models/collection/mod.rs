pub mod _nebula_edges;
pub mod blob;
pub mod datasource;
pub mod filesystem;
pub mod tasks;

use crate::db_management::meilisearch_wait_for_task;
use crate::db_management::nebula_execute;
use crate::db_management::DatabaseSpaceManager;
use crate::db_management::MeilisearchDatabaseHandle;
use crate::migrate::nebula_get_tags_schema;
use crate::migrate::schema::DatabaseSchema;
use crate::models::collection::_nebula_edges::GraphEdgeType;
use charybdis::model::BaseModel;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;

pub fn nebula_sql_insert_vertex(
    table_id: &DatabaseIdentifier,
    schema: DatabaseSchema,
    data: Vec<(String, serde_json::Value)>,
) -> anyhow::Result<String> {
    use anyhow::Context;

    let schema_table = schema
        .tables
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
    use crate::migrate::schema::{DatabaseColumn, DatabaseTable};

    let mut schema = DatabaseSchema {
        tables: std::collections::BTreeMap::new(),
    };
    let table_id = DatabaseIdentifier::new("test").unwrap();
    schema.tables.insert(
        table_id.clone(),
        DatabaseTable {
            name: table_id.clone(),
            columns: vec![
                DatabaseColumn {
                    name: DatabaseIdentifier::new("name").unwrap(),
                    _type: "string".to_string(),
                    primary: true,
                },
                DatabaseColumn {
                    name: DatabaseIdentifier::new("age").unwrap(),
                    _type: "int64".to_string(),
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
        "INSERT VERTEX `test` (`name`, `age`) VALUES \"1\":(\"John\", 30);"
    );
}

fn nebula_sql_insert_edge(
    edge: &GraphEdgeType,
    data: Vec<(String, String)>,
) -> anyhow::Result<String> {
    let edge_name = edge.name;
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
    let edge = GraphEdgeType { name: "test_edge" };
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
    pub fn new(edge: &GraphEdgeType) -> Self {
        Self {
            edge: edge.clone(),
            data: vec![],
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, from: &T1, to: &T2) {
        self.data
            .push((row_pk_hash::<T1>(from), row_pk_hash::<T2>(to)));
    }

    pub async fn execute(self, db_extra: &DatabaseExtraCallbacks) -> anyhow::Result<()> {
        if self.data.is_empty() {
            return Ok(());
        }
        let query = nebula_sql_insert_edge(&self.edge, self.data)?;
        nebula_execute::<()>(&db_extra.collection_id, &query).await?;
        Ok(())
    }
}

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

pub struct DatabaseExtraCallbacks {
    pub collection_id: CollectionId,
    pub nebula_schema: DatabaseSchema,
    pub search_index: std::sync::Arc<
        <meilisearch_sdk::client::Client as DatabaseSpaceManager>::CollectionSessionType,
    >,
}

impl DatabaseExtraCallbacks {
    pub async fn new(c: &CollectionId) -> anyhow::Result<Self> {
        let nebula_schema = nebula_get_tags_schema(c).await?;
        let search_client = MeilisearchDatabaseHandle::collection_session(c).await?;
        Ok(Self {
            collection_id: c.clone(),
            nebula_schema,
            search_index: search_client,
        })
    }

    pub async fn insert<T>(&self, data: &[T]) -> anyhow::Result<()>
    where
        T: BaseModel + serde::Serialize,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static,
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

            let mut obj_map = serde_json::Map::new();
            obj_map.insert("id".to_string(), serde_json::value::Value::String(row_pk));
            obj_map.insert(table_id.to_string(), data_json);
            search_data.push(serde_json::value::Value::Object(obj_map));
        }
        use tokio::time::Duration;

        let _search_result = tokio::time::timeout(
            Duration::from_secs(30),
            self.search_index.add_documents(&search_data, Some("id")),
        )
        .await??;

        let nebula_insert_query =
            nebula_sql_insert_vertex(&table_id, self.nebula_schema.clone(), nebula_data)?;
        nebula_execute::<()>(&self.collection_id, &nebula_insert_query).await?;

        // takes too much time
        // meilisearch_wait_for_task(_search_result).await?;

        Ok(())
    }

    pub async fn delete<T>(&self, data: &[T]) -> anyhow::Result<()>
    where
        T: BaseModel,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static,
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
        nebula_execute::<()>(&self.collection_id, &nebula_delete_query).await?;

        // takes too much time
        meilisearch_wait_for_task(_search_result).await?;

        Ok(())
    }
}

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
