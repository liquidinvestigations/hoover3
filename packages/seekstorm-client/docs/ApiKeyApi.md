# \ApiKeyApi

All URIs are relative to *http://127.0.0.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_apikey_api**](ApiKeyApi.md#create_apikey_api) | **POST** /api/v1/apikey | Create API Key
[**delete_apikey_api**](ApiKeyApi.md#delete_apikey_api) | **DELETE** /api/v1/apikey | Delete API Key
[**get_apikey_indices_info_api**](ApiKeyApi.md#get_apikey_indices_info_api) | **GET** /api/v1/apikey | Get API Key Info



## create_apikey_api

> String create_apikey_api(apikey, create_apikey_api_request)
Create API Key

Creates an API key and returns the Base64 encoded API key. Expects the Base64 encoded master API key in the header. Use the master API key displayed in the server console at startup.  WARNING: make sure to set the MASTER_KEY_SECRET environment variable to a secret, otherwise your generated API keys will be compromised.  For development purposes you may also use the SeekStorm server console command 'create' to create an demo API key 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA='.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_MASTER_API_KEY | [required] |
**create_apikey_api_request** | [**CreateApikeyApiRequest**](CreateApikeyApiRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_apikey_api

> i64 delete_apikey_api(apikey)
Delete API Key

Deletes an API and returns the number of remaining API keys. Expects the Base64 encoded master API key in the header.  WARNING: This will delete all indices and documents associated with the API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_MASTER_API_KEY | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apikey_indices_info_api

> Vec<models::IndexResponseObject> get_apikey_indices_info_api(apikey)
Get API Key Info

Get info about all indices associated with the specified API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | YOUR_SECRET_API_KEY | [required] |

### Return type

[**Vec<models::IndexResponseObject>**](IndexResponseObject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

