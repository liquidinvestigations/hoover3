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

/// DistanceField : DistanceField defines a field for proximity search.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistanceField {
    /// field name of a numeric facet field (currently onyl Point field type supported)
    #[serde(rename = "field")]
    pub field: String,
    /// field name of the distance field we are deriving from the numeric facet field (Point type) and the base (Point type)
    #[serde(rename = "distance")]
    pub distance: String,
    #[serde(rename = "base")]
    pub base: Vec<f64>,
    /// distance unit for the distance field: kilometers or miles
    #[serde(rename = "unit")]
    pub unit: models::DistanceUnit,
}

impl DistanceField {
    /// DistanceField defines a field for proximity search.
    pub fn new(field: String, distance: String, base: Vec<f64>, unit: models::DistanceUnit) -> DistanceField {
        DistanceField {
            field,
            distance,
            base,
            unit,
        }
    }
}

