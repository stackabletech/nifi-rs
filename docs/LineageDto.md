# LineageDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of this lineage query. | [optional]
**uri** | Option<**String**> | The URI for this lineage query for later retrieval and deletion. | [optional]
**submission_time** | Option<**String**> | When the lineage query was submitted. | [optional]
**expiration** | Option<**String**> | When the lineage query will expire. | [optional]
**percent_completed** | Option<**i32**> | The percent complete for the lineage query. | [optional]
**finished** | Option<**bool**> | Whether the lineage query has finished. | [optional]
**request** | Option<[**crate::models::LineageRequestDto**](LineageRequestDTO.md)> |  | [optional]
**results** | Option<[**crate::models::LineageResultsDto**](LineageResultsDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


