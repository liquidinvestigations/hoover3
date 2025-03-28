//! Types and structures related to identifiers and their validation logic.

/// Default keyspace name used in the database
pub const DEFAULT_KEYSPACE_NAME: &str = "hoover3";

/// Represents a unique collection identifier
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub struct CollectionId(String);

impl CollectionId {
    /// Creates a new collection ID after validating the input string
    pub fn new(s: &str) -> Result<CollectionId, anyhow::Error> {
        if s.len() < 3 || s.len() > 32 {
            anyhow::bail!("collection id is too long or too short: {s}");
        }
        if s.contains("__") {
            anyhow::bail!("collection can not have double underscore (__): {s}")
        }
        for c in s.chars() {
            if !(c.is_alphanumeric() || c == '_') {
                anyhow::bail!("collection id not alphanumeric or _: {c}")
            }
        }

        let re = &get_regex().collection_ident;
        if !re.is_match(s) {
            anyhow::bail!(
                "collection identifier '{}' does not match regex {:?}",
                s,
                re
            );
        }
        let c = CollectionId(s.to_string());
        // bail if collection + database name is not ok
        c.database_name()?;

        Ok(c)
    }

    /// Generates the database/keyspace name for this collection
    pub fn database_name(&self) -> anyhow::Result<DatabaseIdentifier> {
        DatabaseIdentifier::new(format!("{}__{}", DEFAULT_KEYSPACE_NAME, self))
    }
}

impl FromStr for CollectionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CollectionId::new(s)
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a unique database identifier
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub struct DatabaseIdentifier(String);

use std::{str::FromStr, sync::OnceLock};
static REGEX: OnceLock<RegexList> = OnceLock::new();

struct RegexList {
    database_ident: regex::Regex,
    collection_ident: regex::Regex,
}

fn get_regex() -> &'static RegexList {
    REGEX.get_or_init(|| RegexList {
        database_ident: regex::Regex::new(r"^[a-z_0-9]{1, 48}$").unwrap(),
        collection_ident: regex::Regex::new(r"^[a-z_0-9]{1, 32}$").unwrap(),
    })
}

impl DatabaseIdentifier {
    /// Creates a new database identifier after validating the input string
    pub fn new(name: impl Into<String>) -> anyhow::Result<Self> {
        let name: String = name.into();
        if name.len() > 48 || name.len() < 3 {
            anyhow::bail!("database identifier '{}' must be 3-48 chars long", name);
        }
        let first_letter = name.chars().next().unwrap();
        if !first_letter.is_alphabetic() || !first_letter.is_ascii() {
            anyhow::bail!("first letter for identifier '{}' must be alphabetic", name);
        }

        if name.starts_with("system") {
            anyhow::bail!(
                "database identifier '{}' must not start with 'system'",
                name
            );
        }
        let re = &get_regex().database_ident;

        if !re.is_match(&name) {
            anyhow::bail!(
                "database identifier '{}' does not match regex {:?}",
                name,
                re
            );
        }

        Ok(Self(name))
    }
}

impl std::fmt::Display for DatabaseIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
impl FromStr for DatabaseIdentifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DatabaseIdentifier::new(s)
    }
}
#[test]
fn test_db_identifier_regex() {
    assert!(DatabaseIdentifier::new("system").is_err());
    assert!(DatabaseIdentifier::new("XXX").is_err());
    assert!(DatabaseIdentifier::new("systom").is_ok());
    assert!(DatabaseIdentifier::new("aaaa").is_ok());
    assert!(DatabaseIdentifier::new("1111aaa").is_err());
    assert!(DatabaseIdentifier::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").is_err()); // len = 49
    assert!(DatabaseIdentifier::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").is_ok()); // len = 48
    assert!(DatabaseIdentifier::new("something_something_123").is_ok());
    assert!(format!("{}", &DatabaseIdentifier::new("xxx").unwrap()) == "xxx");
}
