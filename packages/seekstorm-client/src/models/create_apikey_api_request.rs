/*
 * SeekStorm REST API documentation
 *
 * Search engine library & multi-tenancy server
 *
 * The version of the OpenAPI document: 0.12.11
 * Contact: wolf.garbe@seekstorm.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// CreateApikeyApiRequest : Quota per API key
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateApikeyApiRequest {
    /// number of indices per API key
    #[serde(rename = "indices_max")]
    pub indices_max: i64,
    /// combined index size per API key in MB
    #[serde(rename = "indices_size_max")]
    pub indices_size_max: i64,
    /// combined number of documents in all indices per API key
    #[serde(rename = "documents_max")]
    pub documents_max: i64,
    /// operations per month per API key: index/update/delete/query doc
    #[serde(rename = "operations_max")]
    pub operations_max: i64,
    /// queries per sec per API key
    #[serde(rename = "rate_limit")]
    pub rate_limit: i64,
}

impl CreateApikeyApiRequest {
    /// Quota per API key
    pub fn new(
        indices_max: i64,
        indices_size_max: i64,
        documents_max: i64,
        operations_max: i64,
        rate_limit: i64,
    ) -> CreateApikeyApiRequest {
        CreateApikeyApiRequest {
            indices_max,
            indices_size_max,
            documents_max,
            operations_max,
            rate_limit,
        }
    }
}
