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

/// RangeType : Create query_list and non_unique_query_list blockwise intersection : if the corresponding blocks with a 65k docid range for each term have at least a single docid, then the intersect_docid within a single block is executed  (=segments?) specifies how to count the frequency of numerical facet field values
/// Create query_list and non_unique_query_list blockwise intersection : if the corresponding blocks with a 65k docid range for each term have at least a single docid, then the intersect_docid within a single block is executed  (=segments?) specifies how to count the frequency of numerical facet field values
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RangeType {
    #[serde(rename = "CountWithinRange")]
    CountWithinRange,
    #[serde(rename = "CountAboveRange")]
    CountAboveRange,
    #[serde(rename = "CountBelowRange")]
    CountBelowRange,
}

impl std::fmt::Display for RangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CountWithinRange => write!(f, "CountWithinRange"),
            Self::CountAboveRange => write!(f, "CountAboveRange"),
            Self::CountBelowRange => write!(f, "CountBelowRange"),
        }
    }
}

impl Default for RangeType {
    fn default() -> RangeType {
        Self::CountWithinRange
    }
}
