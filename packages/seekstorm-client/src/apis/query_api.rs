/*
 * SeekStorm REST API documentation
 *
 * Search engine library & multi-tenancy server
 *
 * The version of the OpenAPI document: 0.12.11
 * Contact: wolf.garbe@seekstorm.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`query_index_api_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryIndexApiGetError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_index_api_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryIndexApiPostError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Query results from index with index_id.  Query index via GET is a convenience function, that offers only a limited set of parameters compared to Query Index via POST.
pub async fn query_index_api_get(
    configuration: &configuration::Configuration,
    apikey: &str,
    index_id: i64,
    query: &str,
    offset: i64,
    length: i64,
    realtime: bool,
) -> Result<models::SearchResultObject, Error<QueryIndexApiGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_apikey = apikey;
    let p_index_id = index_id;
    let p_query = query;
    let p_offset = offset;
    let p_length = length;
    let p_realtime = realtime;

    let uri_str = format!(
        "{}/api/v1/index/{index_id}/query",
        configuration.base_path,
        index_id = p_index_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("query", &p_query.to_string())]);
    req_builder = req_builder.query(&[("offset", &p_offset.to_string())]);
    req_builder = req_builder.query(&[("length", &p_length.to_string())]);
    req_builder = req_builder.query(&[("realtime", &p_realtime.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("apikey", p_apikey.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<QueryIndexApiGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Query results from index with index_id  The following parameters are supported: - Result type - Result sorting - Realtime search - Field filter - Fields to include in search results - Distance fields: derived fields from distance calculations - Highlights: keyword-in-context snippets and term highlighting - Query facets: which facets fields to calculate and return at query time - Facet filter: filter facets by field and value - Result sort: sort results by field and direction - Query type default: default query type, if not specified in query
pub async fn query_index_api_post(
    configuration: &configuration::Configuration,
    apikey: &str,
    index_id: i64,
    query_index_api_post_request: models::QueryIndexApiPostRequest,
) -> Result<models::SearchResultObject, Error<QueryIndexApiPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_apikey = apikey;
    let p_index_id = index_id;
    let p_query_index_api_post_request = query_index_api_post_request;

    let uri_str = format!(
        "{}/api/v1/index/{index_id}/query",
        configuration.base_path,
        index_id = p_index_id
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("apikey", p_apikey.to_string());
    req_builder = req_builder.json(&p_query_index_api_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<QueryIndexApiPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
