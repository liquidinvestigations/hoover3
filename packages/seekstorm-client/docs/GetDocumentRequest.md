# GetDocumentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_terms** | Option<**Vec<String>**> | query terms for highlighting | [optional]
**highlights** | Option<[**Vec<models::Highlight>**](Highlight.md)> | which fields to highlight: create keyword-in-context fragments and highlight terms | [optional]
**fields** | Option<**Vec<String>**> | which fields to return | [optional]
**distance_fields** | Option<[**Vec<models::DistanceField>**](DistanceField.md)> | which distance fields to derive and return | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


