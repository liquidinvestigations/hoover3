# \QueryApi

All URIs are relative to *http://127.0.0.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query_index_api_get**](QueryApi.md#query_index_api_get) | **GET** /api/v1/index/{index_id}/query | Query Index
[**query_index_api_post**](QueryApi.md#query_index_api_post) | **POST** /api/v1/index/{index_id}/query | Query Index



## query_index_api_get

> models::SearchResultObject query_index_api_get(apikey, index_id, query, offset, length, realtime)
Query Index

Query results from index with index_id.  Query index via GET is a convenience function, that offers only a limited set of parameters compared to Query Index via POST.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**query** | **String** | query string | [required] |
**offset** | **i64** | result offset | [required] |
**length** | **i64** | result length | [required] |
**realtime** | **bool** | include uncommitted documents | [required] |

### Return type

[**models::SearchResultObject**](SearchResultObject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_index_api_post

> models::SearchResultObject query_index_api_post(apikey, index_id, query_index_api_post_request)
Query Index

Query results from index with index_id  The following parameters are supported: - Result type - Result sorting - Realtime search - Field filter - Fields to include in search results - Distance fields: derived fields from distance calculations - Highlights: keyword-in-context snippets and term highlighting - Query facets: which facets fields to calculate and return at query time - Facet filter: filter facets by field and value - Result sort: sort results by field and direction - Query type default: default query type, if not specified in query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**query_index_api_post_request** | [**QueryIndexApiPostRequest**](QueryIndexApiPostRequest.md) |  | [required] |

### Return type

[**models::SearchResultObject**](SearchResultObject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

