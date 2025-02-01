use crate::db_management::DatabaseSpaceManager;
use crate::db_management::NebulaDatabaseHandle;
use crate::db_management::NebulaDatabaseHandleExt;
use crate::migrate::schema::get_scylla_schema_primary;
use crate::migrate::schema::{DatabaseColumn, DatabaseSchema, DatabaseTable};
use serde::Deserialize;

use anyhow::Result;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use std::collections::BTreeMap;
use tracing::info;

pub async fn _migrate_nebula_collection(c: &CollectionId) -> Result<()> {
    info!("migrating nebula collection {}...", c);
    let session = NebulaDatabaseHandle::collection_session(c).await?;
    for edge_name in crate::models::collection::_nebula_edges::ALL_NEBULA_EDGES {
        session
            .execute::<()>(&format!(
                "CREATE EDGE IF NOT EXISTS `{}` ();",
                edge_name.name
            ))
            .await?;
    }
    let scylla_schema = get_scylla_schema_primary(c).await?;
    // if we already have all the tags, skip the create
    if let Ok(nebula_schema) = nebula_get_tags_schema(c).await {
        if let Ok(_) = check_nebula_schema(c, &scylla_schema, &nebula_schema).await {
            info!(
                "nebula collection {} already has all the tags, skipping create",
                c
            );
            return Ok(());
        }
    }

    let qs = nebula_create_tags_query(&scylla_schema);
    for s in qs {
        info!("nebula create tags query: \n  {}", s);
        session.execute::<()>(&s).await?;
    }
    let nebula_schema = nebula_get_tags_schema(c).await?;
    check_nebula_schema(c, &scylla_schema, &nebula_schema).await?;
    info!("migrating nebula collection {} OK.", c);
    Ok(())
}

async fn check_nebula_schema(
    collection_id: &CollectionId,
    scylla_schema: &DatabaseSchema,
    nebula_schema: &DatabaseSchema,
) -> Result<()> {
    if scylla_schema.tables.len() != nebula_schema.tables.len() {
        anyhow::bail!("scylla schema and nebula schema have different number of tables");
    }
    for (scylla_table, nebula_table) in scylla_schema
        .tables
        .values()
        .zip(nebula_schema.tables.values())
    {
        if scylla_table.name != nebula_table.name {
            anyhow::bail!(
                "scylla table {} and nebula table {} have different names",
                scylla_table.name,
                nebula_table.name
            );
        }
        if scylla_table.columns.len() != nebula_table.columns.len() {
            anyhow::bail!(
                "scylla table {} and nebula table {} have different number of columns",
                scylla_table.name,
                nebula_table.name
            );
        }
        for (scylla_column, nebula_column) in
            scylla_table.columns.iter().zip(nebula_table.columns.iter())
        {
            if scylla_column.name != nebula_column.name {
                anyhow::bail!(
                    "scylla column {} and nebula column {} have different names",
                    scylla_column.name,
                    nebula_column.name
                );
            }
        }
    }

    // Nebula shows tags even if they're not ready to have new vertex inserted.
    // Documentation says to wait 20s -- we're going to attempt insertion onto this tag and see if it works
    let insert_q =
        "INSERT VERTEX `datasource` (`datasource_id`) VALUES \"test___dummy\":(\"test___dummy\");";
    let drop_q = "DELETE VERTEX \"test___dummy\" WITH EDGE;";
    let session = NebulaDatabaseHandle::collection_session(collection_id).await?;
    for _i in 0..60 {
        if session.execute::<()>(&insert_q).await.is_ok() {
            if session.execute::<()>(&drop_q).await.is_ok() {
                return Ok(());
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
    anyhow::bail!("timed out on vertex insert test");
}

pub async fn nebula_get_tags_schema(c: &CollectionId) -> Result<DatabaseSchema> {
    let session = NebulaDatabaseHandle::collection_session(c).await?;
    let mut schema = DatabaseSchema {
        tables: BTreeMap::new(),
    };
    for tag in session.execute::<String>("SHOW TAGS;").await? {
        schema.tables.insert(
            DatabaseIdentifier::new(&tag)?,
            DatabaseTable {
                name: DatabaseIdentifier::new(&tag)?,
                columns: nebula_describe_tag(session.clone(), &tag).await?,
            },
        );
    }
    Ok(schema)
}

async fn nebula_describe_tag(
    session: std::sync::Arc<NebulaDatabaseHandle>,
    tag: &str,
) -> Result<Vec<DatabaseColumn>> {
    let mut columns = vec![];

    #[derive(Deserialize, Debug)]
    pub struct DescribeTagResponse {
        #[serde(rename(deserialize = "Field"))]
        pub _field_name: String,
        #[serde(rename(deserialize = "Type"))]
        pub _field_type: String,
        #[serde(rename(deserialize = "Null"))]
        pub _field_null: String,
        #[serde(rename(deserialize = "Default"))]
        pub _field_default: String,
        #[serde(rename(deserialize = "Comment"))]
        pub _field_comment: String,
    }

    for field in session
        .execute::<DescribeTagResponse>(&format!("DESCRIBE TAG `{}`;", tag))
        .await?
    {
        columns.push(DatabaseColumn {
            name: DatabaseIdentifier::new(&field._field_name)?,
            _type: field._field_type,
            primary: true,
        });
    }
    columns.sort_by_key(|c| c.name.clone());

    Ok(columns)
}

fn nebula_create_tags_query(schema: &DatabaseSchema) -> Vec<String> {
    let mut query = vec![];
    for table in schema.tables.values() {
        query.push(nebula_create_tag_query(&table));
    }
    query
}

fn nebula_create_tag_query(table: &DatabaseTable) -> String {
    let mut query = String::new();
    query.push_str(&format!("CREATE TAG IF NOT EXISTS `{}` (\n", table.name));
    query.push_str(
        &table
            .columns
            .iter()
            .filter(|c| c.primary)
            .map(|c| nebula_create_tag_column_query(&c))
            .collect::<Vec<_>>()
            .join(",\n"),
    );
    query.push_str(");\n");
    query
}

fn nebula_create_tag_column_query(column: &DatabaseColumn) -> String {
    let nebula_type = match column._type.as_str() {
        "boolean" => "bool",
        "int" => "INT32",
        "bigint" => "INT64",
        "smallint" => "INT16",
        "tinyint" => "INT8",
        "float" => "FLOAT",
        "double" => "DOUBLE",
        "text" | "varchar" => "STRING",
        _ => panic!("invalid scylla column type: {}", column._type),
    };
    format!("  `{}` {} NOT NULL", column.name, nebula_type)
}
