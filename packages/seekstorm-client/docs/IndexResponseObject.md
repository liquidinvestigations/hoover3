# IndexResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Index ID | 
**name** | **String** | Index name | 
**schema** | [**std::collections::HashMap<String, models::SchemaField>**](SchemaField.md) |  | 
**indexed_doc_count** | **i32** | Number of indexed documents | 
**operations_count** | **i64** | Number of operations: index, update, delete, queries | 
**query_count** | **i64** | Number of queries, for quotas and billing | 
**version** | **String** | SeekStorm version the index was created with | 
**facets_minmax** | [**std::collections::HashMap<String, models::MinMaxFieldJson>**](MinMaxFieldJson.md) | Minimum and maximum values of numeric facet fields | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


