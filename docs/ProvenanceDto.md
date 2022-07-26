# ProvenanceDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the provenance query. | [optional]
**uri** | Option<**String**> | The URI for this query. Used for obtaining/deleting the request at a later time | [optional]
**submission_time** | Option<**String**> | The timestamp when the query was submitted. | [optional]
**expiration** | Option<**String**> | The timestamp when the query will expire. | [optional]
**percent_completed** | Option<**i32**> | The current percent complete. | [optional]
**finished** | Option<**bool**> | Whether the query has finished. | [optional]
**request** | Option<[**crate::models::ProvenanceRequestDto**](ProvenanceRequestDTO.md)> |  | [optional]
**results** | Option<[**crate::models::ProvenanceResultsDto**](ProvenanceResultsDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


