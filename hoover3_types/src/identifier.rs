pub const DEFAULT_KEYSPACE_NAME: &str = "hoover3";

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub struct CollectionId(String);

impl CollectionId {
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

    pub fn database_name(&self) -> anyhow::Result<DatabaseIdentifier> {
        DatabaseIdentifier::new(format!("{}__{}", DEFAULT_KEYSPACE_NAME, self.to_string()))
    }
}

impl ToString for CollectionId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub struct DatabaseIdentifier(String);

use std::sync::OnceLock;
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

impl ToString for DatabaseIdentifier {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

use std::fmt;
impl fmt::Display for &DatabaseIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
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
