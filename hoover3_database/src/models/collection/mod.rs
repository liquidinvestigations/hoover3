pub mod _nebula_edges;
pub mod blob;
pub mod datasource;
pub mod filesystem;

use crate::db_management::DatabaseSpaceManager;
use crate::db_management::NebulaDatabaseHandle;
use crate::db_management::NebulaDatabaseHandleExt;
use crate::migrate::nebula_get_tags_schema;
use crate::migrate::schema::DatabaseSchema;
use charybdis::model::BaseModel;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;

pub struct DatabaseExtraCallbacks {
    pub nebula_handle: std::sync::Arc<NebulaDatabaseHandle>,
    pub nebula_schema: DatabaseSchema,
}

pub fn nebula_sql_insert_vertex(
    table_id: &DatabaseIdentifier,
    schema: DatabaseSchema,
    data: Vec<(String, serde_json::Value)>,
) -> anyhow::Result<String> {
    use anyhow::Context;

    let schema_table = schema
        .tables
        .get(&table_id)
        .context("table not found in schema")?;
    let column_defs = schema_table
        .columns
        .iter()
        .map(|c| format!("`{}`", c.name.to_string()))
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

impl DatabaseExtraCallbacks {
    pub async fn new(c: &CollectionId) -> anyhow::Result<Self> {
        let nebula_handle = NebulaDatabaseHandle::collection_session(c).await?;
        let nebula_schema = nebula_get_tags_schema(c).await?;
        Ok(Self {
            nebula_handle,
            nebula_schema,
        })
    }

    pub fn row_pk_hash<T: BaseModel>(&self, data: &T) -> anyhow::Result<String>
    where
        T: BaseModel,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static,
    {
        use hoover3_types::stable_hash::stable_hash;
        let x = stable_hash(&data.primary_key_values())?;
        let table_name = T::DB_MODEL_NAME;
        Ok(format!("{table_name}_{x}"))
    }

    pub async fn insert<T: BaseModel>(&self, data: &[T]) -> anyhow::Result<()>
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
        let mut v = vec![];
        for d in data.iter() {
            let nebula_pk = self.row_pk_hash(d)?;
            let data_json = serde_json::to_value(d)?;
            v.push((nebula_pk, data_json));
        }

        let query = nebula_sql_insert_vertex(&table_id, self.nebula_schema.clone(), v)?;

        tracing::info!("nebula_sql_insert_vertex:\n {query}");
        self.nebula_handle.execute::<()>(&query).await?;

        Ok(())
    }

    pub async fn delete<T: BaseModel>(&self, data: &[T]) -> anyhow::Result<()>
    where
        T: BaseModel,
        <T as BaseModel>::PrimaryKey: serde::Serialize,
        <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
        <T as BaseModel>::PrimaryKey: 'static,
    {        if data.is_empty() {
        return Ok(());
    }
        let pks = data
            .iter()
            .map(|d| self.row_pk_hash(d))
            .filter_map(|pk| pk.ok().map(|pk| format!("\"{pk}\"")))
            .collect::<Vec<String>>()
            .join(",");
        let query = format!(
            "
            DELETE VERTEX {pks} WITH EDGE;
            "
        );
        tracing::info!("nebula_sql_delete_vertex:\n {query}");

        self.nebula_handle.execute::<()>(&query).await?;

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
