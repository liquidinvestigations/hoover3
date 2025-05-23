//! ScyllaDB database management module that implements connection pooling and query execution
//! functionality. Provides implementations for the DatabaseSpaceManager trait and handles
//! keyspace operations.
use hoover3_types::identifier::DEFAULT_KEYSPACE_NAME;
use scylla::{CachingSession, SessionBuilder};
use std::collections::HashMap;
use std::{collections::hash_map::RandomState, sync::Arc};
use tokio::sync::{OnceCell, RwLock};
use tracing::info;

use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};

pub struct ScyllaConnection(CachingSession<RandomState>);

/// Scylla database handle type alias.
pub type ScyllaDatabaseHandle = ScyllaConnection;

impl DatabaseSpaceManager for ScyllaDatabaseHandle {
    type CollectionSessionType = Self;
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        static SCYLLA_CONNECTION_GLOBAL: OnceCell<Arc<ScyllaConnection>> = OnceCell::const_new();
        Ok(SCYLLA_CONNECTION_GLOBAL
            .get_or_init(|| async {
                _open_new_session(
                    DatabaseIdentifier::new(DEFAULT_KEYSPACE_NAME).unwrap(),
                    true,
                )
                .await
                .unwrap()
            })
            .await
            .clone())
    }
    async fn collection_session(_c: &CollectionId) -> Result<Arc<Self>, anyhow::Error> {
        static HASH: OnceCell<RwLock<HashMap<CollectionId, Arc<ScyllaConnection>>>> =
            OnceCell::const_new();
        let h = HASH
            .get_or_init(|| async move { RwLock::new(HashMap::new()) })
            .await;
        // try to fetch from hashmap
        {
            let h = h.read().await;
            if let Some(s) = h.get(_c) {
                return Ok(s.clone());
            }
        }
        // if not found, open new session
        let s = {
            let mut h = h.write().await;
            let s = _open_new_session(_c.database_name()?, false).await?;
            h.insert(_c.clone(), s.clone());
            s
        };
        Ok(s)
    }

    async fn space_exists(&self, name: &DatabaseIdentifier) -> anyhow::Result<bool> {
        let query = format!(
            "select keyspace_name from system_schema.keyspaces where keyspace_name = '{}' ;",
            name
        );
        info!("QUERY: {}", query);

        let data = self.execute_unpaged(query, &[]).await?;
        let rows = data.into_rows_result()?.rows::<(String,)>()?.next();

        if let Some(Ok((keyspace_name,))) = rows {
            if keyspace_name == name.to_string() {
                return Ok(true);
            }
        }
        Ok(false)
    }
    async fn list_spaces(&self) -> anyhow::Result<Vec<DatabaseIdentifier>> {
        let query = "select keyspace_name from system_schema.keyspaces;";
        info!("QUERY: {}", query);
        let mut v = vec![];

        for row in self
            .execute_unpaged(query, &[])
            .await?
            .into_rows_result()?
            .rows::<(String,)>()?
        {
            let name = row?.0;
            if let Ok(name) = DatabaseIdentifier::new(&name) {
                v.push(name);
            } else {
                info!("skipped scylla space {}", name);
            }
        }
        Ok(v)
    }
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if self.space_exists(name).await? {
            return Ok(());
        }
        info!("SCYLLA: CREATE SPACE {:?}", name);
        let query = format!(
            "CREATE KEYSPACE IF NOT EXISTS \"{}\"
        WITH REPLICATION =    {{'class' : 'NetworkTopologyStrategy', 'datacenter1' : 1}}
         AND TABLETS = {{'enabled': false}}",
            name
        );
        self.execute_unpaged(query, &[]).await?;

        // wait until space now exists
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let mut i = 0;
        while !self.space_exists(name).await? {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            i += 1;
            if i > 300 {
                anyhow::bail!("space {} does not exist after 30 seconds", name);
            }
        }

        Ok(())
    }
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if !self.space_exists(name).await? {
            return Ok(());
        }
        info!("SCYLLA: DROP SPACE {:?}", name);
        let query = format!("DROP KEYSPACE IF EXISTS \"{}\";", name);
        self.execute_unpaged(query, &[]).await?;

        // wait until space does not exist
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let mut i = 0;
        while self.space_exists(name).await? {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            i += 1;
            if i > 300 {
                anyhow::bail!(
                    "drop_space error: space {} does still exist after 30 seconds",
                    name
                );
            }
        }

        Ok(())
    }
    async fn migrate_collection_space(_c: &CollectionId) -> Result<(), anyhow::Error> {
        info!("scylla_migrate_collection {}", _c.to_string());
        let session = ScyllaDatabaseHandle::collection_session(_c).await?;
        let space_name = _c.database_name()?.to_string();

        // collect codes from inventory and extra files that will be listed by hand
        let schema_code = crate::models::collection::get_all_charybdis_codes();
        let extra_codes = crate::migrate::get_extra_charybdis_codes();

        // we put the codes in a temp file in some temp dir
        // we put the codes in a temp file in some temp dir
        let temp_dir = std::env::temp_dir().join("hoover3_charybdis_codes");
        std::fs::create_dir_all(&temp_dir).unwrap();
        info!("temp_dir: {}", temp_dir.to_string_lossy());
        let temp_file = temp_dir.join("charybdis_codes.rs");
        std::fs::write(temp_file, schema_code.join("\n")).unwrap();
        for (i, item) in extra_codes.iter().enumerate() {
            let temp_file = temp_dir.join(format!("charybdis_extra_codes_{}.rs", i));
            std::fs::write(temp_file, item).unwrap();
        }
        // we run the migration on that temp dir

        // let schema_json = get_scylla_code_schema_json()?;

        let migration = charybdis::migrate::MigrationBuilder::new()
            .keyspace(space_name.clone())
            .drop_and_replace(true)
            .current_dir(temp_dir.to_string_lossy().into_owned())
            .build(session.get_session())
            .await;

        migration.run().await;
        // TODO: check / wait until migration was successful
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        info!("execute collection {space_name} migration OK");

        Ok(())
    }
}

impl std::ops::Deref for ScyllaConnection {
    type Target = CachingSession<RandomState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

async fn _open_new_session(
    space: DatabaseIdentifier,
    create_if_missing: bool,
) -> anyhow::Result<Arc<ScyllaConnection>> {
    let space = space.to_string();
    let uri = "127.0.0.1:6642";
    info!("connect {}", uri);

    let s1 = match (
        SessionBuilder::new()
            .known_node(uri)
            .use_keyspace(&space, false)
            .compression(Some(scylla::frame::Compression::Lz4))
            .build()
            .await,
        create_if_missing,
    ) {
        (Ok(s), _) => s,
        (
            Err(scylla::transport::errors::NewSessionError::DbError(
                scylla::transport::errors::DbError::Invalid,
                _b,
            )),
            true,
        ) => {
            info!("creating keyspace {}", &space);
            // keyspace does not exist -- connect without and create it
            let s2 = SessionBuilder::new().known_node(uri).build().await?;
            s2.query_unpaged(
                format!(
                    r#"CREATE KEYSPACE IF NOT EXISTS {space}
                    WITH REPLICATION = {{
                        'class' : 'NetworkTopologyStrategy','datacenter1' : 1
                    }}
                    AND TABLETS = {{'enabled': false}}"#
                ),
                &[],
            )
            .await?;
            s2.query_unpaged(format!("USE {space};"), &[]).await?;

            info!("USE {}", space);
            s2
        }
        (Err(_e), _) => {
            anyhow::bail!("connection error: {:?}", _e);
        }
    };

    info!("connect session ok.");

    Ok(Arc::new(ScyllaConnection(CachingSession::from(s1, 1000))))
}
