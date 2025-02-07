# \PdfFileApi

All URIs are relative to *http://127.0.0.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_file_api**](PdfFileApi.md#get_file_api) | **GET** /api/v1/index/{index_id}/file/{document_id} | Get PDF file
[**index_file_api**](PdfFileApi.md#index_file_api) | **POST** /api/v1/index/{index_id}/file | Index PDF file



## get_file_api

> Vec<i32> get_file_api(apikey, index_id, document_id)
Get PDF file

Get PDF file from index with index_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**index_id** | **i64** | index id | [required] |
**document_id** | **i64** | document id | [required] |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_file_api

> i32 index_file_api(apikey, file, date, index_id, request_body)
Index PDF file

Index PDF file (byte array) to the index with the specified apikey and index_id, and return the number of indexed docs. - Converts PDF to a JSON document with \"title\", \"body\", \"url\" and \"date\" fields and indexes it. - extracts title from metatag, or first line of text, or from filename - extracts creation date from metatag, or from file creation date (Unix timestamp: the number of seconds since 1 January 1970) - copies all ingested pdf files to \"files\" subdirectory in index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |
**file** | **String** | filepath from header for JSON 'url' field | [required] |
**date** | **String** | date (timestamp) from header, as fallback for JSON 'date' field, if PDF date meta tag unaivailable | [required] |
**index_id** | **i64** | index id | [required] |
**request_body** | [**Vec<i32>**](i32.md) |  | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

