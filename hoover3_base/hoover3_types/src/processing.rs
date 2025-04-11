//! Types related to processing pipeline
use serde::{Deserialize, Serialize};

use crate::{
    filesystem::FsScanResult,
    identifier::{CollectionId, DatabaseIdentifier},
};

/// Result of processing a datasource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProcessDatasourceTaskResult {
    /// The collection id.
    pub collection_id: CollectionId,
    /// The datasource id.
    pub datasource_id: DatabaseIdentifier,
    /// The scan result.
    pub scan: FsScanResult,
    /// The processing result.
    pub process: CollectionProcessingResult,
}

/// Result of processing a number of de-duplicated blobs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollectionProcessingResult {
    /// The collection id.
    pub collection_id: CollectionId,

    /// Plan page id count for small pages.
    pub small_page_count: i32,

    /// Results for small pages
    pub small_page_results: ProcessPageResult,

    /// Plan page id count for large pages.
    pub large_page_count: i32,

    /// Results for large pages
    pub large_page_results: ProcessPageResult,
}

/// The result of processing a page.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProcessPageResult {
    /// The number of items in the page.
    pub item_count: i32,
    /// The number of items that were successfully processed.
    pub item_success: i32,
    /// The number of items that were not processed correctly.
    pub item_errors: i32,
}
