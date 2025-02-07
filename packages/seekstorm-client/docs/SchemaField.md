# SchemaField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | unique name of a field | 
**stored** | **bool** | only stored fields are returned in the search results | 
**indexed** | **bool** | only indexed fields can be searched | 
**field_type** | [**models::FieldType**](FieldType.md) | type of a field | 
**facet** | Option<**bool**> | optional faceting for a field Faceting can be enabled both for string field type and numerical field types. both numerical and string fields can be indexed (indexed=true) and stored (stored=true) in the json document, but with field_facet=true they are additionally stored in a binary format, for fast faceting and sorting without docstore access (decompression, deserialization) | [optional]
**boost** | Option<**f32**> | optional custom weight factor for Bm25 ranking | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


