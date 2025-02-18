//! ClickHouse database management module that provides functionality for creating, managing,
//! and interacting with ClickHouse databases. Implements the DatabaseSpaceManager trait for
//! handling database spaces and sessions.
use super::{DatabaseIdentifier, DatabaseSpaceManager};
use clickhouse::{Client, Row};
use hoover3_types::identifier::CollectionId;
use serde::Deserialize;
use std::{env, sync::Arc};
use tokio::sync::OnceCell;

/// ClickHouse database handle type alias.
pub type ClickhouseDatabaseHandle = Client;

fn clickhouse_url() -> String {
    env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://localhost:8123".to_owned())
}

impl DatabaseSpaceManager for ClickhouseDatabaseHandle {
    type CollectionSessionType = Self;
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        static CLICKHOUSE_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();
        Ok(CLICKHOUSE_CLIENT
            .get_or_init(|| async {
                let url = clickhouse_url();
                Arc::new(
                    Client::default()
                        .with_url(url)
                        .with_user("hoover3")
                        .with_password("hoover3"),
                )
            })
            .await
            .clone())
    }
    async fn collection_session(c: &super::CollectionId) -> Result<Arc<Self>, anyhow::Error> {
        // TODO cache these creds
        let c = Client::default()
            .with_url(clickhouse_url())
            .with_database(c.database_name()?.to_string())
            .with_user("hoover3")
            .with_password("hoover3");
        Ok(Arc::new(c))
    }
    async fn space_exists(&self, name: &DatabaseIdentifier) -> anyhow::Result<bool> {
        let query = format!("EXISTS DATABASE {}", name);
        let rv = self.query(&query).fetch_one::<u8>().await?;
        Ok(rv != 0)
    }
    async fn list_spaces(&self) -> anyhow::Result<Vec<DatabaseIdentifier>> {
        #[derive(Deserialize, Debug, Row)]
        pub struct DbEntry {
            #[serde(rename(deserialize = "name"))]
            pub name: String,
        }
        Ok(self
            .query("SHOW DATABASES")
            .fetch_all::<DbEntry>()
            .await?
            .iter()
            .filter_map(|c| DatabaseIdentifier::new(&c.name).ok())
            .collect::<Vec<_>>())
    }
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        let query = format!("CREATE DATABASE IF NOT EXISTS {} ;", name);
        self.query(&query)
            .with_option("wait_end_of_query", "1")
            .execute()
            .await?;
        Ok(())
    }
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        let query = format!("DROP DATABASE IF EXISTS {} ;", name);
        self.query(&query)
            .with_option("wait_end_of_query", "1")
            .execute()
            .await?;
        Ok(())
    }

    async fn migrate_collection_space(_c: &CollectionId) -> Result<(), anyhow::Error> {
        // TODO: implement
        Ok(())
    }
}
