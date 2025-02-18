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

/// QueryType : Specifies the default QueryType: The following query types are supported: - **Union** (OR, disjunction), - **Intersection** (AND, conjunction), - **Phrase** (\"\"), - **Not** (-).  The default QueryType is superseded if the query parser detects that a different query type is specified within the query string (+ - \"\").
/// Specifies the default QueryType: The following query types are supported: - **Union** (OR, disjunction), - **Intersection** (AND, conjunction), - **Phrase** (\"\"), - **Not** (-).  The default QueryType is superseded if the query parser detects that a different query type is specified within the query string (+ - \"\").
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QueryType {
    #[serde(rename = "Union")]
    Union,
    #[serde(rename = "Intersection")]
    Intersection,
    #[serde(rename = "Phrase")]
    Phrase,
    #[serde(rename = "Not")]
    Not,
}

impl std::fmt::Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Union => write!(f, "Union"),
            Self::Intersection => write!(f, "Intersection"),
            Self::Phrase => write!(f, "Phrase"),
            Self::Not => write!(f, "Not"),
        }
    }
}

impl Default for QueryType {
    fn default() -> QueryType {
        Self::Union
    }
}
