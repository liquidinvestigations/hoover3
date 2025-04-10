//! Types and structures related to stable hashing.

/// Computes a stable hash for a given serializable key.
/// The hash should be the same across frontend and backend platforms.
pub fn stable_hash<K>(key: &K) -> anyhow::Result<String>
where
    K: serde::Serialize + 'static + Send + for<'a> serde::Deserialize<'a>,
{
    let key_bytes = bincode::serialize(key)?;
    let key_hash = stable_hash::fast_stable_hash(&key_bytes);
    let key_hash = format!("{:X}", key_hash);
    Ok(key_hash)
}
