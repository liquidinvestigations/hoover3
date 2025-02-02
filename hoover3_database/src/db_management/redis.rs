use std::future::Future;

use tokio::sync::OnceCell;
use tracing::info;
use tracing::warn;

pub async fn redis_client() -> &'static redis::Client {
    static CLIENT: OnceCell<redis::Client> = OnceCell::const_new();
    CLIENT
        .get_or_init(move || async move { redis::Client::open("redis://127.0.0.1:6379/").unwrap() })
        .await
}

pub async fn redis_lockmanager() -> &'static rslock::LockManager {
    static CLIENT: OnceCell<rslock::LockManager> = OnceCell::const_new();
    CLIENT
        .get_or_init(move || async move {
            rslock::LockManager::new(vec![redis_client().await.get_connection_info().clone()])
        })
        .await
}

pub async fn redis_connection() -> anyhow::Result<redis::aio::MultiplexedConnection> {
    static CLIENT: OnceCell<redis::aio::MultiplexedConnection> = OnceCell::const_new();
    Ok(CLIENT
        .get_or_init(move || async move {
            redis_client()
                .await
                .get_multiplexed_tokio_connection()
                .await
                .unwrap()
        })
        .await
        .clone())
}

pub async fn with_redis_lock<F>(redis_lock_id: &str, func: F) -> anyhow::Result<F::Output>
where
    F: Future + Send + 'static,
    F::Output: 'static + Send + Sync,
{
    let redis_lock_id = format!("hoover3_lock__{}", redis_lock_id);
    use std::time::Duration;
    let lock_ttl = 60.0; // 1 min
    let finalize_retry_count = 5 * (2 * 60 * 24); // 5 days

    let rl = redis_lockmanager().await;
    // info!("lockmanager init OK for {redis_lock_id}");

    let mut retry_count = 0;
    let mut acq_retry_wait_secs = 0.016;
    let max_acq_retry = 14; // 2 min
    let lock = loop {
        if let Ok(lock) = rl
            .lock(redis_lock_id.as_bytes(), Duration::from_secs_f64(lock_ttl))
            .await
        {
            break lock;
        }
        tokio::time::sleep(Duration::from_secs_f64(acq_retry_wait_secs)).await;
        retry_count += 1;
        acq_retry_wait_secs *= 2.0;
        if retry_count >= max_acq_retry {
            anyhow::bail!("could not acquire lock after 2min",);
        }
    };

    // info!("Lock {redis_lock_id} acquired!");

    retry_count = 0;
    let mut f = tokio::spawn(func);
    let v = loop {
        let wait = tokio::time::sleep(tokio::time::Duration::from_secs_f64(lock_ttl / 2.0));
        tokio::pin!(wait);
        use futures_util::future::Either;
        f = match futures::future::select(f, wait).await {
            Either::Left((result, _wait)) => {
                // info!("locked {redis_lock_id} computation finalized;");
                break result;
            }
            Either::Right((_value, _orig)) => {
                info!("lock {redis_lock_id} sleep lapsed.");
                if rl
                    .extend(&lock, Duration::from_secs_f64(lock_ttl))
                    .await
                    .is_ok()
                {
                    info!("Lock {redis_lock_id} extended!");
                } else {
                    info!("Failed to extend the {redis_lock_id} lock.");
                    _orig.abort();
                    anyhow::bail!(
                        "failed to extend lock after {} seconds; aborting task",
                        lock_ttl * retry_count as f64
                    )
                }
                _orig
            }
        };
        retry_count += 1;
        if retry_count > finalize_retry_count {
            rl.unlock(&lock).await;
            f.abort();
            anyhow::bail!(
                "locked task not finished after {} seconds; aborting",
                lock_ttl * retry_count as f64
            )
        }
    };

    rl.unlock(&lock).await;
    // info!("Lock {redis_lock_id} released!");
    Ok(v?)
}

pub async fn drop_redis_cache<K>(redis_cache_id: &str, key: &K) -> anyhow::Result<()>
where
    K: serde::Serialize + 'static + Send + for<'a> serde::Deserialize<'a> + Clone,
{
    let key_hash = hoover3_types::stable_hash::stable_hash(key)?;
    let redis_cache_id = format!("hoover3_cache_data__{}__{}", redis_cache_id, key_hash);
    let mut conn = redis_connection().await?;
    let empty = Vec::<u8>::new();
    redis::cmd("SET")
        .arg(redis_cache_id)
        .arg(empty)
        .arg("EX")
        .arg(1)
        .exec_async(&mut conn)
        .await?;
    Ok(())
}

pub async fn with_redis_cache<K, F, T>(
    redis_cache_id: &str,
    ttl_sec: u32,
    func: impl FnOnce(K) -> F + Send + 'static,
    key: &K,
) -> anyhow::Result<T>
where
    K: serde::Serialize + 'static + Send + for<'a> serde::Deserialize<'a> + Clone,
    F: Future<Output = anyhow::Result<T>> + 'static + Send,
    T: 'static + Send + Sync,
    T: serde::Serialize,
    T: for<'a> serde::Deserialize<'a>,
{
    _with_redis_cache(
        redis_cache_id,
        ttl_sec,
        move |c| async move {
            func(c)
                .await
                .map_err(|e| format!("_get_all_collections: {e}"))
        },
        key,
    )
    .await?
    .map_err(|e| anyhow::anyhow!("{redis_cache_id}: {e}"))
}

async fn _with_redis_cache<K, F>(
    redis_cache_id: &str,
    ttl_sec: u32,
    func: impl FnOnce(K) -> F + Send + 'static,
    key: &K,
) -> anyhow::Result<F::Output>
where
    K: serde::Serialize + 'static + Send + for<'a> serde::Deserialize<'a> + Clone,
    F: Future + 'static + Send,
    F::Output: 'static + Send + Sync,
    F::Output: serde::Serialize,
    F::Output: for<'a> serde::Deserialize<'a>,
{
    const MAX_CACHE_LINE_SIZE: usize = 1024 * 1024 * 64; // 64MB limit; redis limit is 512MB
    let key_hash = hoover3_types::stable_hash::stable_hash(key)?;
    let redis_lock_id = format!("cache__{}__{}", redis_cache_id, key_hash);
    let redis_cache_id = format!("hoover3_cache_data__{}__{}", redis_cache_id, key_hash);

    // first try reading from cache
    let mut conn = redis_connection().await?;
    if let Ok(_cache_hit) = redis::cmd("GET")
        .arg(&redis_cache_id)
        .query_async::<Vec<u8>>(&mut conn)
        .await
    {
        if let Ok(_cache_hit) = bincode::deserialize::<F::Output>(&_cache_hit) {
            // info!("CACHE HIT 1: {redis_cache_id}!");
            return Ok(_cache_hit);
        }
    }

    // then obtain lock
    let key = key.clone();
    with_redis_lock(&redis_lock_id, async move {
        // we have the lock; if we waited for it that means someone else
        // added a value in the cache. Let's try fetching that again
        let mut conn = redis_connection().await?;
        if let Ok(_cache_hit) = redis::cmd("GET")
            .arg(&redis_cache_id)
            .query_async::<Vec<u8>>(&mut conn)
            .await
        {
            if let Ok(_cache_hit) = bincode::deserialize::<F::Output>(&_cache_hit) {
                // info!("CACHE HIT 2: {redis_cache_id}!");
                return Ok(_cache_hit);
            }
        }
        // info!("CACHE MISS: {redis_cache_id}!");
        // We're sure there is not cached value. We can compute it now.
        let f = tokio::spawn(func(key.clone()));
        let rv = f.await?;
        let bytes_to_cache = bincode::serialize(&rv)?;
        if bytes_to_cache.len() > MAX_CACHE_LINE_SIZE {
            warn!(
                "cache response for {redis_cache_id} is {} bytes, bigger than limit {}",
                bytes_to_cache.len(),
                MAX_CACHE_LINE_SIZE
            );
            return Ok(rv);
        }

        // info!("CACHE SET: {redis_cache_id}!");
        redis::cmd("SET")
            .arg(redis_cache_id)
            .arg(bytes_to_cache)
            .arg("EX")
            .arg(ttl_sec)
            .exec_async(&mut conn)
            .await?;
        Ok(rv)
    })
    .await?
}

#[tokio::test]
async fn test_redis_exp_cache() {
    async fn test_fn(i: u32) -> anyhow::Result<u32> {
        Ok(i)
    }
    let x = with_redis_cache("test_fn", 1, test_fn, &6).await.unwrap();
    assert_eq!(x, 6);
    let x = with_redis_cache("test_fn", 1, test_fn, &6).await.unwrap();
    assert_eq!(x, 6);
    // make sure it's there
    let mut conn = redis_connection().await.unwrap();
    let x = redis::cmd("GET")
        .arg("hoover3_cache_data__test_fn__F8CAF7A1A0BE51BF2A0A21FCF196AB04")
        .query_async::<Vec<u8>>(&mut conn)
        .await;
    info!("{:#?}", x);
    assert_eq!(
        x.unwrap(),
        bincode::serialize(&Result::<u32, String>::Ok(6_u32)).unwrap()
    );

    // sleep 2s -- ensure expired from redis
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // make sure it's not there
    let x = redis::cmd("GET")
        .arg("hoover3_cache_data__test_fn__F8CAF7A1A0BE51BF2A0A21FCF196AB04")
        .query_async::<Vec<u8>>(&mut conn)
        .await;
    info!("{:#?}", x);

    assert!(x.is_err() || x.unwrap().is_empty());
}

#[tokio::test]
async fn test_redis_drop_cache() {
    async fn test_fn2(i: u32) -> anyhow::Result<u32> {
        Ok(i)
    }

    let x = with_redis_cache("test_fn2", 3, test_fn2, &6).await.unwrap();
    assert_eq!(x, 6);
    // make sure it's there
    let mut conn = redis_connection().await.unwrap();
    let x = redis::cmd("GET")
        .arg("hoover3_cache_data__test_fn2__F8CAF7A1A0BE51BF2A0A21FCF196AB04")
        .query_async::<Vec<u8>>(&mut conn)
        .await;
    info!("{:#?}", x);
    assert_eq!(
        x.unwrap(),
        bincode::serialize(&Result::<u32, String>::Ok(6_u32)).unwrap()
    );

    // DROP CACHE HERE
    drop_redis_cache("test_fn2", &6_u32).await.unwrap();

    // make sure it's not there
    let x = redis::cmd("GET")
        .arg("hoover3_cache_data__test_fn2__F8CAF7A1A0BE51BF2A0A21FCF196AB04")
        .query_async::<Vec<u8>>(&mut conn)
        .await;
    info!("{:#?}", x);

    assert!(x.is_err() || x.unwrap().is_empty());
}
