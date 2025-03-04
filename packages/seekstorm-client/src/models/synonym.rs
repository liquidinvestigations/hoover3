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

/// Synonym : Defines synonyms for terms per index.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Synonym {
    /// List of terms that are synonyms.
    #[serde(rename = "terms")]
    pub terms: Vec<String>,
    /// Creates alternative versions of documents where in each copy a term is replaced with one of its synonyms. Doesn't impact the query latency, but does increase the index size. Multi-way synonyms (default): all terms are synonyms of each other. One-way synonyms: only the first term is a synonym of the following terms, but not vice versa. E.g. [street, avenue, road] will result in searches for street to return documents containing any of the terms street, avenue or road, but searches for avenue will only return documents containing avenue, but not documents containing street or road. Currently only single terms without spaces are supported. Synonyms are supported in result highlighting. The synonyms that were created with the synonyms parameter in create_index are stored in synonyms.json in the index directory contains Can be manually modified, but becomes effective only after restart and only for newly indexed documents.
    #[serde(rename = "multiway", skip_serializing_if = "Option::is_none")]
    pub multiway: Option<bool>,
}

impl Synonym {
    /// Defines synonyms for terms per index.
    pub fn new(terms: Vec<String>) -> Synonym {
        Synonym {
            terms,
            multiway: None,
        }
    }
}
