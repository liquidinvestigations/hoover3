/*
cargo run -p nebula-demo-tokio --bin nebula_demo_tokio_v3_bb8_graph_pool 127.0.0.1 9669 root 'password'
*/

use std::{env, sync::Arc};
use tracing::info;

use anyhow::Context;
use bb8_nebula::{
    graph::GraphClientConfiguration, impl_tokio::v3::graph::new_graph_connection_manager,
    GraphConnectionManager,
};
use fbthrift_transport::AsyncTransportConfiguration;

use bb8::{Pool, PooledConnection};
use hoover3_types::identifier::DEFAULT_KEYSPACE_NAME;
use nebula_client::v3::{graph::GraphQueryOutput, GraphQuery as _, GraphTransportResponseHandler};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio::sync::{Mutex, OnceCell};

use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};

type TManager = GraphConnectionManager<
    async_compat::Compat<tokio::net::TcpStream>,
    tokio::time::Sleep,
    nebula_client::v3::GraphTransportResponseHandler,
    nebula_client::VersionV3,
>;
type TPool2 = Pool<TManager>;
type TSession = PooledConnection<'static, TManager>;
pub type NebulaDatabaseHandle = Mutex<TSession>;
use std::collections::HashMap;
use tokio::sync::RwLock;


use tokio::time::Duration;
const NEBULA_QUERY_TIMEOUT: Duration = Duration::from_secs(60);

async fn nebula_execute_once<T: DeserializeOwned + std::fmt::Debug>(
    collection: &CollectionId,
    query: &str,
) -> Result<Vec<T>, anyhow::Error> {
    let handle = tokio::time::timeout(
        NEBULA_QUERY_TIMEOUT,
        NebulaDatabaseHandle::collection_session(collection),
    )
    .await
    .context("nebula get session timeout")??;
    let query = query.as_bytes().to_vec();
    let mut session = tokio::time::timeout(NEBULA_QUERY_TIMEOUT, handle.lock())
        .await
        .context("nebula session lock timeout")?;
    let result = tokio::time::timeout(NEBULA_QUERY_TIMEOUT, session.query_as::<T>(&query))
        .await
        .context("nebula query execution timeout")?;
    Ok(result?.data_set)
}

pub async fn nebula_execute<T: DeserializeOwned + std::fmt::Debug>(
    collection: &CollectionId,
    query: &str,
) -> Result<Vec<T>, anyhow::Error> {
    let retry_count = 3;
    for i in 1..=retry_count {
        let res = nebula_execute_once(collection, query).await;
        if res.is_ok() {
            return res;
        }
        if i == retry_count {
            anyhow::bail!("nebula query failed after {retry_count} retries: {res:?}");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(
            NEBULA_SESSION_CACHE_DURATION_SECONDS,
        ))
        .await;
    }
    unreachable!();
}

const NEBULA_SESSION_CACHE_DURATION_SECONDS: f64 = 5.0;

impl DatabaseSpaceManager for NebulaDatabaseHandle {
    type CollectionSessionType = Self;
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        use anyhow::Context;
        Ok(Arc::new(Mutex::new(
            _open_new_session(DEFAULT_KEYSPACE_NAME)
                .await
                .context("_open_new_session")?,
        )))
    }
    async fn collection_session(_c: &CollectionId) -> Result<Arc<Self>, anyhow::Error> {
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        // TODO - cache in memory hashmap and drop old collection sessions

        #[allow(clippy::type_complexity)]
        static HASH: OnceCell<RwLock<HashMap<CollectionId, (f64, Arc<NebulaDatabaseHandle>)>>> =
            OnceCell::const_new();
        let h = HASH
            .get_or_init(|| async move { RwLock::new(HashMap::new()) })
            .await;
        // try to fetch from hashmap
        {
            let h = h.read().await;
            if let Some(s) = h.get(_c) {
                let created_at = s.0;
                if current_timestamp - created_at < NEBULA_SESSION_CACHE_DURATION_SECONDS {
                    return Ok(s.1.clone());
                }
            }
        }
        // if not found, open new session
        let s = {
            let mut h = h.write().await;
            let s = {
                Arc::new(Mutex::new(
                    _open_new_session(&_c.database_name()?.to_string()).await?,
                ))
            };
            h.insert(_c.clone(), (current_timestamp, s.clone()));
            s
        };
        Ok(s)
    }

    async fn list_spaces(&self) -> anyhow::Result<Vec<DatabaseIdentifier>> {
        let res = {
            let mut session = self.lock().await;

            session.show_spaces().await?
        };
        info!("show spaces: {res:?}");
        Ok(res
            .data_set
            .iter()
            .filter_map(|x| DatabaseIdentifier::new(&x.name).ok())
            .collect::<Vec<_>>())
    }
    async fn space_exists(&self, name: &DatabaseIdentifier) -> anyhow::Result<bool> {
        let query = format!(" SHOW CREATE SPACE `{}`;", name);
        let query = query.as_bytes().to_vec();

        #[derive(Deserialize, Debug)]
        pub struct CreateSpace {
            #[serde(rename(deserialize = "Space"))]
            pub _space: String,
            #[serde(rename(deserialize = "Create Space"))]
            pub _create_space: String,
        }

        let res = {
            let mut session = self.lock().await;
            let res: Result<GraphQueryOutput<CreateSpace>, _> = session.query_as(&query).await;
            res
        };
        Ok(res.is_ok())
    }
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        // nebula create_space is quite flaky, let's try 3 times
        let retry = 3;
        for _i in 1..=retry {
            if self.space_exists(name).await? {
                return Ok(());
            }

            let query = format!(
                "
                CREATE SPACE IF NOT EXISTS  `{}`
                (partition_num=64, replica_factor=1,
                vid_type=FIXED_STRING(64)) ;
                ",
                name
            );
            info!("nebula create space query: {}", query);
            let query = query.as_bytes().to_vec();

            let res = {
                let mut session = self.lock().await;
                let res: Result<_, _> = session.query(&query).await;
                res
            };
            match res {
                Ok(_v) => {
                    return Ok(());
                }
                Err(e) => {
                    if _i == retry {
                        anyhow::bail!("nebula create space result: {e}");
                    }
                    continue;
                }
            }
        }
        unreachable!();
    }
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if !self.space_exists(name).await? {
            return Ok(());
        }

        let query = format!("  DROP SPACE IF EXISTS  `{}`  ;", name);
        let query = query.as_bytes().to_vec();

        let res = {
            let mut session = self.lock().await;
            let res: Result<_, _> = session.query(&query).await;
            res
        };

        res?;
        Ok(())
    }
}

static NEBULA_POOL: OnceCell<Arc<TPool2>> = OnceCell::const_new();
async fn _open_new_session(space: &str) -> anyhow::Result<TSession> {
    info!("_open_new_session({:?})", space);

    let pool = NEBULA_POOL
        .get_or_init(|| async {
            info!("pool init for nebula space ({:?})", space);

            let domain = env::var("NEBULA_DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_owned());

            let port: u16 = env::var("NEBULA_PORT")
                .unwrap_or_else(|_| "9669".to_owned())
                .parse()
                .unwrap();

            let username = env::var("NEBULA_USERNAME").unwrap_or_else(|_| "root".to_owned());
            let password = env::var("NEBULA_PASSWORD").unwrap_or_else(|_| "password".to_owned());
            let space = env::var("NEBULA_DEFAULT_SPACE").ok();

            info!("nebula connection pool: {domain} {port} {username} {password} {space:?}",);

            let client_configuration =
                GraphClientConfiguration::new(domain, port, username, password, space.clone());
            let transport_configuration =
                AsyncTransportConfiguration::new(GraphTransportResponseHandler);
            let manager =
                new_graph_connection_manager(client_configuration, transport_configuration);
            let pool = bb8::Pool::builder()
                .max_size(16)
                .build(manager)
                .await
                .expect("create nebula pool");

            info!("pool OK for nebula space ({:?}).", space);
            Arc::new(pool)
        })
        .await;

    info!("starting nebula session for {space}...");

    let mut session = pool.get().await?;

    let sql_use = format!("USE {space}").as_bytes().to_vec();
    // use nebula_fbthrift_common_v3::types::ErrorCode;
    match session.query(&sql_use).await {
        Ok(_) => {}
        Err(nebula_client::v3::graph::GraphQueryError::ResponseError(error_code, Some(vec))) => {
            if error_code.0 == -1005i32 {
                // magic const from the package we don't have
                let err_msg = String::from_utf8_lossy(&vec);
                if err_msg.starts_with("SpaceNotFound") {
                    info!("nebula space not found ; creating...");
                    let sql_create = format!(
                        "
                        CREATE SPACE IF NOT EXISTS {space}
                        (partition_num = 16,
                        replica_factor = 1,
                        vid_type = FIXED_STRING(64)
                        );
                        "
                    )
                    .as_bytes()
                    .to_vec();
                    session.query(&sql_create).await?;
                    for _ in 0..6 {
                        if session.query(&sql_use).await.is_ok() {
                            return Ok(session);
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
                    }
                    anyhow::bail!("use {space}: could not use newly created space.");
                }
            }
        }
        Err(_e) => {
            anyhow::bail!("use {space}: graph query error: \n'{_e}'.\n")
        }
    }

    info!(" nebula session OK for {space}.");
    Ok(session)
}
