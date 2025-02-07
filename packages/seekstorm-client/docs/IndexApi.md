# \IndexApi

All URIs are relative to *http://127.0.0.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commit_index_api**](IndexApi.md#commit_index_api) | **PATCH** /api/v1/index/{index_id} | Commit Index
[**create_index_api**](IndexApi.md#create_index_api) | **POST** /api/v1/index | Create Index
[**delete_index_api**](IndexApi.md#delete_index_api) | **DELETE** /api/v1/index/{index_id} | Delete Index
[**get_index_info_api**](IndexApi.md#get_index_info_api) | **GET** /api/v1/index/{index_id} | Get Index Info



## commit_index_api

> i64 commit_index_api(apikey, index_id)
Commit Index

Commit moves indexed documents from the intermediate uncompressed data structure (array lists/HashMap, queryable by realtime search) in RAM to the final compressed data structure (roaring bitmap) on Mmap or disk - which is persistent, more compact, with lower query latency and allows search with realtime=false. Commit is invoked automatically each time 64K documents are newly indexed as well as on close_index (e.g. server quit). There is no way to prevent this automatic commit by not manually invoking it. But commit can also be invoked manually at any time at any number of newly indexed documents. commit is a **hard commit** for persistence on disk. A **soft commit** for searchability is invoked implicitly with every index_doc, i.e. the document can immediately searched and included in the search results if it matches the query AND the query paramter realtime=true is enabled.  **Use commit with caution, as it is an expensive operation**. **Usually, there is no need to invoke it manually**, as it is invoked automatically every 64k documents and when the index is closed with close_index. Before terminating the program, always call close_index (commit), otherwise all documents indexed since last (manual or automatic) commit are lost. There are only 2 reasons that justify a manual commit: 1. if you want to search newly indexed documents without using realtime=true for search performance reasons or 2. if after indexing new documents there won't be more documents indexed (for some time),    so there won't be (soon) a commit invoked automatically at the next 64k threshold or close_index,    but you still need immediate persistence guarantees on disk to protect against data loss in the event of a crash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_index_api

> i64 create_index_api(apikey, create_index_api_request)
Create Index

Create an index within the directory associated with the specified API key and return the index_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**create_index_api_request** | [**CreateIndexApiRequest**](CreateIndexApiRequest.md) |  | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_index_api

> i64 delete_index_api(apikey, index_id)
Delete Index

Delete an index within the directory associated with the specified API key and return the number of remaining indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_index_info_api

> models::IndexResponseObject get_index_info_api(apikey, index_id)
Get Index Info

Get index Info from index with index_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |

### Return type

[**models::IndexResponseObject**](IndexResponseObject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

