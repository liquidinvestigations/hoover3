use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataBackend {
    LocalDisk,
    S3,
    WebDav,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataAccessSettings {
    pub local_disk: Option<LocalDiskSettings>,
    pub s3: Option<S3Settings>,
    pub webdav: Option<WebDavSettings>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocalDiskSettings {
    pub root_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct S3Settings {
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WebDavSettings {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl DataAccessSettings {
    pub fn new() -> Self {
        Self {
            local_disk: None,
            s3: None,
            webdav: None,
        }
    }

    pub fn set_local_disk(&mut self, root_path: PathBuf) {
        self.local_disk = Some(LocalDiskSettings { root_path });
    }

    pub fn set_s3(&mut self, bucket: String, access_key: String, secret_key: String) {
        self.s3 = Some(S3Settings {
            bucket,
            access_key,
            secret_key,
        });
    }

    pub fn set_webdav(&mut self, url: String, username: String, password: String) {
        self.webdav = Some(WebDavSettings {
            url,
            username,
            password,
        });
    }
}

impl Default for DataAccessSettings {
    fn default() -> Self {
        Self::new()
    }
}
