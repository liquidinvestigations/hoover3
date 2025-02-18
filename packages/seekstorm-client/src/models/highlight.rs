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

/// Highlight : Specifies the number and size of fragments (snippets, summaries) to generate from each specified field to provide a \"keyword in context\" (KWIC) functionality. With highlight_markup the matching query terms within the fragments can be highlighted with HTML markup.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Highlight {
    /// Specifies the field from which the fragments  (snippets, summaries) are created.
    #[serde(rename = "field")]
    pub field: String,
    /// Allows to specifiy multiple highlight result fields from the same source field, leaving the original field intact, Default: if name is empty then field is used instead, i.e the original field is overwritten with the highlight.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If 0/default then return the full original text without fragmenting.
    #[serde(rename = "fragment_number", skip_serializing_if = "Option::is_none")]
    pub fragment_number: Option<i32>,
    /// Specifies the length of a highlight fragment. The default 0 returns the full original text without truncating, but still with highlighting if highlight_markup is enabled.
    #[serde(rename = "fragment_size", skip_serializing_if = "Option::is_none")]
    pub fragment_size: Option<i32>,
    /// if true, the matching query terms within the fragments are highlighted with HTML markup **\\<b\\>term\\</b\\>**.
    #[serde(rename = "highlight_markup", skip_serializing_if = "Option::is_none")]
    pub highlight_markup: Option<bool>,
}

impl Highlight {
    /// Specifies the number and size of fragments (snippets, summaries) to generate from each specified field to provide a \"keyword in context\" (KWIC) functionality. With highlight_markup the matching query terms within the fragments can be highlighted with HTML markup.
    pub fn new(field: String) -> Highlight {
        Highlight {
            field,
            name: None,
            fragment_number: None,
            fragment_size: None,
            highlight_markup: None,
        }
    }
}
