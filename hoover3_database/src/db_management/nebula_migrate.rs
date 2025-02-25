use crate::db_management::{nebula_execute_retry, with_redis_cache};
use crate::models::collection::_nebula_edges::get_all_nebula_edge_types;
use anyhow::Result;
use hoover3_types::db_schema::{
    DatabaseColumn, DatabaseColumnType, DatabaseTable, GraphEdgeType, NebulaDatabaseSchema,
    ScyllaDatabaseSchema,
};
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use serde::Deserialize;
use std::collections::BTreeMap;
use tracing::info;

/// API Client method to migrate the Nebula database for a collection.
pub async fn migrate_nebula_collection(c: &CollectionId) -> Result<()> {
    info!("migrating nebula collection {}...", c);

    for edge_name in get_all_nebula_edge_types() {
        nebula_execute_retry::<()>(
            c,
            &format!("CREATE EDGE IF NOT EXISTS `{}` ();", edge_name.name),
        )
        .await?;
    }
    let scylla_schema = hoover3_types::db_schema::get_scylla_schema_from_inventory();
    // if we already have all the tags, skip the create
    if let Ok(nebula_schema) = _query_nebula_get_schema(c.clone()).await {
        if check_nebula_schema(c, &scylla_schema, &nebula_schema)
            .await
            .is_ok()
        {
            info!(
                "nebula collection {} already has all the tags, skipping create",
                c
            );
            return Ok(());
        }
    }

    let qs = nebula_create_tags_query(&scylla_schema);
    for s in qs {
        println!("nebula create tags query: \n  {}", s);
        nebula_execute_retry::<()>(c, &s).await?;
    }
    let nebula_schema = _query_nebula_get_schema(c.clone()).await?;
    check_nebula_schema(c, &scylla_schema, &nebula_schema).await?;
    info!("migrating nebula collection {} OK.", c);
    Ok(())
}

async fn check_nebula_schema(
    collection_id: &CollectionId,
    scylla_schema: &ScyllaDatabaseSchema,
    nebula_schema: &NebulaDatabaseSchema,
) -> Result<()> {
    if scylla_schema.tables.len() != nebula_schema.tags.len() {
        anyhow::bail!(
            "scylla schema {:#?} and nebula schema {:#?} have different number of tables",
            scylla_schema,
            nebula_schema
        );
    }
    for (scylla_table, nebula_table) in scylla_schema
        .tables
        .values()
        .zip(nebula_schema.tags.values())
    {
        let scylla_columns = scylla_table
            .columns
            .iter()
            .filter(|c| c.primary)
            .collect::<Vec<_>>();
        if scylla_table.name != nebula_table.name {
            anyhow::bail!(
                "scylla table {} and nebula table {} have different names",
                scylla_table.name,
                nebula_table.name
            );
        }
        if scylla_columns.len() != nebula_table.columns.len() {
            anyhow::bail!(
                "scylla table {} and nebula table {} have different number of columns",
                scylla_table.name,
                nebula_table.name
            );
        }
        for (scylla_column, nebula_column) in scylla_columns.iter().zip(nebula_table.columns.iter())
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
    for _i in 0..60 {
        if nebula_execute_retry::<()>(collection_id, insert_q)
            .await
            .is_ok()
            && nebula_execute_retry::<()>(collection_id, drop_q)
                .await
                .is_ok()
        {
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
    anyhow::bail!("timed out on vertex insert test");
}

/// API Client method to get the Nebula database schema for a collection. Query the database with 60s cache.
pub async fn query_nebula_schema(c: &CollectionId) -> Result<NebulaDatabaseSchema> {
    let c = c.clone();
    with_redis_cache("query_nebula_schema", 60, _query_nebula_get_schema, &c).await
}

/// Query database schema without caching.
async fn _query_nebula_get_schema(c: CollectionId) -> Result<NebulaDatabaseSchema> {
    tracing::info!("nebula_get_schema {}", c.to_string());
    let mut schema = NebulaDatabaseSchema {
        tags: BTreeMap::new(),
        edges: nebula_execute_retry::<String>(&c, "SHOW EDGES;")
            .await?
            .into_iter()
            .filter_map(|s| DatabaseIdentifier::new(&s).ok())
            .map(|v| GraphEdgeType { name: v })
            .collect(),
    };
    for tag in nebula_execute_retry::<String>(&c, "SHOW TAGS;").await? {
        schema.tags.insert(
            DatabaseIdentifier::new(&tag)?,
            DatabaseTable {
                name: DatabaseIdentifier::new(&tag)?,
                columns: nebula_describe_tag(&c, &tag).await?,
            },
        );
    }
    Ok(schema)
}

async fn nebula_describe_tag(c: &CollectionId, tag: &str) -> Result<Vec<DatabaseColumn>> {
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

    for field in
        nebula_execute_retry::<DescribeTagResponse>(c, &format!("DESCRIBE TAG `{}`;", tag)).await?
    {
        columns.push(DatabaseColumn {
            name: DatabaseIdentifier::new(&field._field_name)?,
            _type: DatabaseColumnType::from_nebula_type(&field._field_type)?,
            primary: true,
        });
    }
    columns.sort_by_key(|c| c.name.clone());

    Ok(columns)
}

fn nebula_create_tags_query(schema: &ScyllaDatabaseSchema) -> Vec<String> {
    let mut query = vec![];
    for table in schema.tables.values() {
        query.push(nebula_create_tag_query(table));
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
            .map(nebula_create_tag_column_query)
            .collect::<Vec<_>>()
            .join(",\n"),
    );
    query.push_str(");\n");
    query
}

fn nebula_create_tag_column_query(column: &DatabaseColumn) -> String {
    let nebula_type = column._type.to_nebula_type().unwrap();
    format!("  `{}` {} NOT NULL", column.name, nebula_type)
}
