use hoover3_types::identifier::DEFAULT_KEYSPACE_NAME;
use scylla::{CachingSession, SessionBuilder};
use std::collections::HashMap;
use std::{collections::hash_map::RandomState, sync::Arc};
use tokio::sync::{OnceCell, RwLock};
use tracing::info;

use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};

pub struct ScyllaConnection(CachingSession<RandomState>);

pub type ScyllaDatabaseHandle = ScyllaConnection;

impl ScyllaDatabaseHandle {
    pub async fn open_session(space: DatabaseIdentifier) -> anyhow::Result<Arc<ScyllaConnection>> {
        _open_new_session(space).await
    }
}

impl DatabaseSpaceManager for ScyllaDatabaseHandle {
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        static SCYLLA_CONNECTION_GLOBAL: OnceCell<Arc<ScyllaConnection>> = OnceCell::const_new();
        Ok(SCYLLA_CONNECTION_GLOBAL
            .get_or_init(|| async {
                _open_new_session(DatabaseIdentifier::new(DEFAULT_KEYSPACE_NAME).unwrap())
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
            let s = _open_new_session(_c.database_name()?).await?;
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
        let query = format!("CREATE KEYSPACE IF NOT EXISTS \"{}\" WITH REPLICATION =    {{'class' : 'NetworkTopologyStrategy', 'datacenter1' : 1}}", name);
        self.execute_unpaged(query, &[]).await?;

        Ok(())
    }
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if !self.space_exists(name).await? {
            return Ok(());
        }
        info!("SCYLLA: DROP SPACE {:?}", name);
        let query = format!("DROP KEYSPACE IF EXISTS \"{}\";", name);
        self.execute_unpaged(query, &[]).await?;

        Ok(())
    }
}

impl std::ops::Deref for ScyllaConnection {
    type Target = CachingSession<RandomState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

async fn _open_new_session(space: DatabaseIdentifier) -> anyhow::Result<Arc<ScyllaConnection>> {
    let space = space.to_string();
    let uri = "127.0.0.1:6642";
    info!("connect {}", uri);

    let s1 = match SessionBuilder::new()
        .known_node(uri)
        .use_keyspace(&space, false)
        .compression(Some(scylla::frame::Compression::Lz4))
        .build()
        .await
    {
        Ok(s) => s,
        Err(scylla::transport::errors::NewSessionError::DbError(
            scylla::transport::errors::DbError::Invalid,
            _b,
        )) => {
            info!("creating keyspace {}", &space);
            // keyspace does not exist -- connect without and create it
            let s2 = SessionBuilder::new().known_node(uri).build().await?;
            s2.query_unpaged(
                format!(
                    r#"CREATE KEYSPACE IF NOT EXISTS {space}
                    WITH REPLICATION = {{
                        'class' : 'NetworkTopologyStrategy','datacenter1' : 1
                    }}"#
                ),
                &[],
            )
            .await?;
            s2.query_unpaged(format!("USE {space};"), &[]).await?;

            info!("USE {}", space);
            s2
        }
        Err(_e) => {
            anyhow::bail!("other connection error: {:?}", _e);
        }
    };

    info!("connect session ok.");

    Ok(Arc::new(ScyllaConnection(CachingSession::from(s1, 1000))))
}
