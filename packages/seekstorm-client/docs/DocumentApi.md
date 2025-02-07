# \DocumentApi

All URIs are relative to *http://127.0.0.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_document_by_object_api**](DocumentApi.md#delete_document_by_object_api) | **DELETE** /api/v1/index/{index_id}/doc | Delete Document(s) by Request Object
[**delete_document_by_parameter_api**](DocumentApi.md#delete_document_by_parameter_api) | **DELETE** /api/v1/index/{index_id}/doc/{document_id} | Delete Document
[**get_document_api**](DocumentApi.md#get_document_api) | **GET** /api/v1/index/{index_id}/doc/{document_id} | Get Document
[**index_document_api**](DocumentApi.md#index_document_api) | **POST** /api/v1/index/{index_id}/doc | Index Document(s)
[**update_document_api**](DocumentApi.md#update_document_api) | **PATCH** /api/v1/index/{index_id}/doc | Update Document(s)



## delete_document_by_object_api

> i32 delete_document_by_object_api(apikey, index_id, search_request_object)
Delete Document(s) by Request Object

Delete document by document_id, by array of document_id (bulk), by query (SearchRequestObject) from index with index_id, or clear all documents from index.  Immediately effective, indpendent of commit. Index space used by deleted documents is not reclaimed (until compaction is implemented), but result_count_total is updated. By manually deleting the delete.bin file the deleted documents can be recovered (until compaction).  Deleted documents impact performance, especially but not limited to counting (Count, TopKCount). They also increase the size of the index (until compaction is implemented). For minimal query latency delete index and reindexing documents is preferred over deleting documents (until compaction is implemented). BM25 scores are not updated (until compaction is implemented), but the impact is minimal.  Document ID can by obtained by search. When deleting by query (SearchRequestObject), it is advised to perform a dry run search first, to see which documents will be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**search_request_object** | [**SearchRequestObject**](SearchRequestObject.md) | Specifies the document(s) to delete by different request objects - 'clear' : delete all documents in index (clear index) - u64 : delete single doc ID - [u64] : delete array of doc ID  - SearchRequestObject : delete documents by query | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_document_by_parameter_api

> i32 delete_document_by_parameter_api(apikey, index_id, document_id)
Delete Document

Delete document by document_id from index with index_id  Document ID can by obtained by search. Immediately effective, indpendent of commit. Index space used by deleted documents is not reclaimed (until compaction is implemented), but result_count_total is updated. By manually deleting the delete.bin file the deleted documents can be recovered (until compaction).  Deleted documents impact performance, especially but not limited to counting (Count, TopKCount). They also increase the size of the index (until compaction is implemented). For minimal query latency delete index and reindexing documents is preferred over deleting documents (until compaction is implemented). BM25 scores are not updated (until compaction is implemented), but the impact is minimal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**document_id** | **i64** | document id | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_api

> std::collections::HashMap<String, serde_json::Value> get_document_api(apikey, index_id, document_id, get_document_request)
Get Document

Get document from index with index_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**document_id** | **i64** | document id | [required] |
**get_document_request** | [**GetDocumentRequest**](GetDocumentRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_document_api

> i32 index_document_api(apikey, index_id, request_body)
Index Document(s)

Index a JSON document or an array of JSON documents (bulk), each consisting of arbitrary key-value pairs to the index with the specified apikey and index_id, and return the number of indexed docs.  Index documents enables true real-time search (as opposed to near realtime.search): When in query_index the parameter `realtime` is set to `true` then indexed, but uncommitted documents are immediately included in the search results, without requiring a commit or refresh. Therefore a explicit commit_index is almost never required, as it is invoked automatically after 64k documents are indexed or on close_index for persistence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**request_body** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | JSON document or array of JSON documents, each consisting of key-value pairs | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_document_api

> i32 update_document_api(apikey, index_id, request_body)
Update Document(s)

Update a JSON document or an array of JSON documents (bulk), each consisting of arbitrary key-value pairs to the index with the specified apikey and index_id, and return the number of indexed docs.  Update document is a combination of delete_document and index_document. All current limitations of delete_document apply.  Update documents enables true real-time search (as opposed to near realtime.search): When in query_index the parameter `realtime` is set to `true` then indexed, but uncommitted documents are immediately included in the search results, without requiring a commit or refresh. Therefore a explicit commit_index is almost never required, as it is invoked automatically after 64k documents are indexed or on close_index for persistence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**request_body** | [**Vec<serde_json::Value>**](serde_json::Value.md) | Tuple of (doc_id, JSON document) or array of tuples (doc_id, JSON documents), each JSON document consisting of arbitrary key-value pairs | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

