# Synonym

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**terms** | **Vec<String>** | List of terms that are synonyms. | 
**multiway** | Option<**bool**> | Creates alternative versions of documents where in each copy a term is replaced with one of its synonyms. Doesn't impact the query latency, but does increase the index size. Multi-way synonyms (default): all terms are synonyms of each other. One-way synonyms: only the first term is a synonym of the following terms, but not vice versa. E.g. [street, avenue, road] will result in searches for street to return documents containing any of the terms street, avenue or road, but searches for avenue will only return documents containing avenue, but not documents containing street or road. Currently only single terms without spaces are supported. Synonyms are supported in result highlighting. The synonyms that were created with the synonyms parameter in create_index are stored in synonyms.json in the index directory contains Can be manually modified, but becomes effective only after restart and only for newly indexed documents. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


