use crate::db_management::DatabaseSpaceManager;
use crate::db_management::NebulaDatabaseHandle;
use crate::db_management::NebulaDatabaseHandleExt;
use crate::migrate::schema::get_scylla_schema;
use crate::migrate::schema::{DatabaseColumn, DatabaseSchema, DatabaseTable};

use anyhow::Result;
use hoover3_types::identifier::CollectionId;
use tracing::info;

pub async fn _migrate_nebula_collection(c: &CollectionId) -> Result<()> {
    info!("migrating nebula collection {}...", c);
    let qs = nebula_create_tags_query(&get_scylla_schema(c).await?);
    let session = NebulaDatabaseHandle::collection_session(c).await?;
    for s in qs {
        info!("nebula create tags query: \n  {}", s);
        session.execute::<()>(s).await?;
    }
    info!("migrating nebula collection {} OK.", c);
    Ok(())
}

fn nebula_create_tags_query(schema: &DatabaseSchema) -> Vec<String> {
    let mut query = vec![];
    for table in schema.tables.iter() {
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
