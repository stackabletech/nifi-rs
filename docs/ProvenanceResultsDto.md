# ProvenanceResultsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provenance_events** | Option<[**Vec<crate::models::ProvenanceEventDto>**](ProvenanceEventDTO.md)> | The provenance events that matched the search criteria. | [optional]
**total** | Option<**String**> | The total number of results formatted. | [optional]
**total_count** | Option<**i64**> | The total number of results. | [optional]
**generated** | Option<**String**> | Then the search was performed. | [optional]
**oldest_event** | Option<**String**> | The oldest event available in the provenance repository. | [optional]
**time_offset** | Option<**i32**> | The time offset of the server that's used for event time. | [optional]
**errors** | Option<**Vec<String>**> | Any errors that occurred while performing the provenance request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


