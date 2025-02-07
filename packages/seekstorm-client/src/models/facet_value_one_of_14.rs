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

/// FacetValueOneOf14 : Point value: latitude/lat, longitude/lon
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacetValueOneOf14 {
    #[serde(rename = "Point")]
    pub point: Vec<f64>,
}

impl FacetValueOneOf14 {
    /// Point value: latitude/lat, longitude/lon
    pub fn new(point: Vec<f64>) -> FacetValueOneOf14 {
        FacetValueOneOf14 {
            point,
        }
    }
}

