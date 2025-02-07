# SearchRequestObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | **String** | Query string, search operators + - \"\" are recognized. | 
**offset** | **i32** | Offset of search results to return. | 
**length** | Option<**i32**> | Number of search results to return. | [optional]
**result_type** | Option<[**models::ResultType**](ResultType.md)> |  | [optional]
**realtime** | Option<**bool**> | True realtime search: include indexed, but uncommitted documents into search results. | [optional]
**highlights** | Option<[**Vec<models::Highlight>**](Highlight.md)> |  | [optional]
**field_filter** | Option<**Vec<String>**> | Specify field names where to search at querytime, whereas SchemaField.indexed is set at indextime. If empty then all indexed fields are searched. | [optional]
**fields** | Option<**Vec<String>**> |  | [optional]
**distance_fields** | Option<[**Vec<models::DistanceField>**](DistanceField.md)> |  | [optional]
**query_facets** | Option<[**Vec<models::QueryFacet>**](QueryFacet.md)> |  | [optional]
**facet_filter** | Option<[**Vec<models::FacetFilter>**](FacetFilter.md)> |  | [optional]
**result_sort** | Option<[**Vec<models::ResultSort>**](ResultSort.md)> | Sort field and order:  Search results are sorted by the specified facet field, either in ascending or descending order. If no sort field is specified, then the search results are sorted by rank in descending order per default. Multiple sort fields are combined by a \"sort by, then sort by\"-method (\"tie-breaking\"-algorithm). The results are sorted by the first field, and only for those results where the first field value is identical (tie) the results are sub-sorted by the second field, until the n-th field value is either not equal or the last field is reached. A special _score field (BM25x), reflecting how relevant the result is for a given search query (phrase match, match in title etc.) can be combined with any of the other sort fields as primary, secondary or n-th search criterium. Sort is only enabled on facet fields that are defined in schema at create_index!  Examples: - result_sort = vec![ResultSort {field: \"price\".into(), order: SortOrder::Descending, base: FacetValue::None},ResultSort {field: \"language\".into(), order: SortOrder::Ascending, base: FacetValue::None}]; - result_sort = vec![ResultSort {field: \"location\".into(),order: SortOrder::Ascending, base: FacetValue::Point(vec![38.8951, -77.0364])}]; | [optional]
**query_type_default** | Option<[**models::QueryType**](QueryType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


