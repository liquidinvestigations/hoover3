//! Models for the processing plugin.

#![allow(missing_docs)]
use hoover3_macro::model;

/// Model for storing metadata extracted from a blob.
#[model]
pub struct BlobExtractedMetadataRow {
    /// The sha3-256 hash of the blob.
    #[model(primary(partition))]
    #[model(search(index))]
    pub blob_sha3_256: String,

    /// Type of metadata provider, e.g. "tika"
    #[model(primary(clustering))]
    #[model(search(facet))]
    pub meta_provider: String,

    /// Metadata key
    #[model(primary(clustering))]
    #[model(search(facet))]
    pub meta_key: String,

    /// Entry index
    #[model(primary(clustering))]
    #[model(search(facet))]
    pub list_index: i32,

    /// Metadata value
    #[model(search(index))]
    pub value: String,
}

/// Model for storing the content of a blob.
#[model]
pub struct BlobExtractedContentRow {
    /// The sha3-256 hash of the blob.
    #[model(primary(partition))]
    #[model(search(index))]
    pub blob_sha3_256: String,

    /// The source for the content, e.g. "tika"
    #[model(primary(clustering))]
    #[model(search(facet))]
    pub content_source: String,

    /// Entry index
    #[model(primary(clustering))]
    #[model(search(facet))]
    pub list_index: i32,

    /// The content of the blob.
    #[model(search(index))]
    pub content: String,

    /// Length  of content string, in bytes
    #[model(search(facet))]
    pub content_length: i32,
}
