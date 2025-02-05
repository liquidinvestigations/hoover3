use std::env;
use std::sync::Arc;

use s3::creds::Credentials;
use s3::{Bucket, BucketConfiguration, Region};
use tokio::sync::OnceCell;

use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};

#[derive(Debug, Clone)]
pub struct S3Configs {
    region: Region,
    creds: Credentials,
}

impl S3Configs {
    fn _get_bucket(&self, name: &DatabaseIdentifier) -> Box<Bucket> {
        let name = name.to_string().replace("_", "-");
        Bucket::new(&name, self.region.clone(), self.creds.clone())
            .expect("cannot create bucket")
            .with_path_style()
    }
}

/// Seaweed database handle type alias.
pub type S3DatabaseHandle = S3Configs;

impl DatabaseSpaceManager for S3DatabaseHandle {
    type CollectionSessionType = Self;
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        static S3_CONFIGS: OnceCell<Arc<S3Configs>> = OnceCell::const_new();
        Ok(S3_CONFIGS
            .get_or_init(|| async {
                let url = env::var("S3_URL").unwrap_or_else(|_| "http://localhost:8333".to_owned());
                let access_key =
                    env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "some_access_key1".to_owned());
                let secret_key =
                    env::var("S3_SECRET_KEY").unwrap_or_else(|_| "some_secret_key1".to_owned());

                let region = Region::Custom {
                    region: "eu-central-1".to_owned(),
                    endpoint: url,
                };
                let credentials =
                    Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
                        .expect("not accepting creds");

                Arc::new(S3Configs {
                    region,
                    creds: credentials,
                })
            })
            .await
            .clone())
    }
    async fn collection_session(_c: &CollectionId) -> Result<Arc<Self>, anyhow::Error> {
        // s3 does not have a client-per-database config
        Self::global_session().await
    }
    async fn space_exists(&self, name: &DatabaseIdentifier) -> anyhow::Result<bool> {
        let bucket = self._get_bucket(name);
        let exists = bucket.exists().await?;
        Ok(exists)
    }
    async fn list_spaces(&self) -> anyhow::Result<Vec<DatabaseIdentifier>> {
        Ok(
            Bucket::list_buckets(self.region.clone(), self.creds.clone())
                .await?
                .bucket_names()
                .filter_map(|x| DatabaseIdentifier::new(x.replace("-", "_")).ok())
                .collect(),
        )
    }
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if self.space_exists(name).await? {
            return Ok(());
        }
        let name = name.to_string().replace("_", "-");
        let new_bucket = Bucket::create_with_path_style(
            &name,
            self.region.clone(),
            self.creds.clone(),
            BucketConfiguration::default(),
        )
        .await?;
        if new_bucket.response_code != 200 {
            anyhow::bail!(
                "got status code {} from bucket create, wanted 200",
                new_bucket.response_code
            );
        }

        Ok(())
    }
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if !self.space_exists(name).await? {
            return Ok(());
        }
        Bucket::delete(&self._get_bucket(name)).await?;
        Ok(())
    }
}
