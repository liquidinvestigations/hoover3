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

/// SimilarityType : Similarity type defines the scoring and ranking of the search results: - Bm25f: considers documents composed from several fields, with different field lengths and importance - Bm25fProximity: considers term proximity, e.g. for implicit phrase search with improved relevancy
/// Similarity type defines the scoring and ranking of the search results: - Bm25f: considers documents composed from several fields, with different field lengths and importance - Bm25fProximity: considers term proximity, e.g. for implicit phrase search with improved relevancy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SimilarityType {
    #[serde(rename = "Bm25f")]
    Bm25f,
    #[serde(rename = "Bm25fProximity")]
    Bm25fProximity,

}

impl std::fmt::Display for SimilarityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bm25f => write!(f, "Bm25f"),
            Self::Bm25fProximity => write!(f, "Bm25fProximity"),
        }
    }
}

impl Default for SimilarityType {
    fn default() -> SimilarityType {
        Self::Bm25f
    }
}

