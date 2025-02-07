# Highlight

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Specifies the field from which the fragments  (snippets, summaries) are created. | 
**name** | Option<**String**> | Allows to specifiy multiple highlight result fields from the same source field, leaving the original field intact, Default: if name is empty then field is used instead, i.e the original field is overwritten with the highlight. | [optional]
**fragment_number** | Option<**i32**> | If 0/default then return the full original text without fragmenting. | [optional]
**fragment_size** | Option<**i32**> | Specifies the length of a highlight fragment. The default 0 returns the full original text without truncating, but still with highlighting if highlight_markup is enabled. | [optional]
**highlight_markup** | Option<**bool**> | if true, the matching query terms within the fragments are highlighted with HTML markup **\\<b\\>term\\</b\\>**. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


