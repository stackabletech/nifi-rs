# ListingRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id for this listing request. | [optional]
**uri** | Option<**String**> | The URI for future requests to this listing request. | [optional]
**submission_time** | Option<**String**> | The timestamp when the query was submitted. | [optional]
**last_updated** | Option<**String**> | The last time this listing request was updated. | [optional]
**percent_completed** | Option<**i32**> | The current percent complete. | [optional]
**finished** | Option<**bool**> | Whether the query has finished. | [optional]
**failure_reason** | Option<**String**> | The reason, if any, that this listing request failed. | [optional]
**max_results** | Option<**i32**> | The maximum number of FlowFileSummary objects to return | [optional]
**state** | Option<**String**> | The current state of the listing request. | [optional]
**queue_size** | Option<[**crate::models::QueueSizeDto**](QueueSizeDTO.md)> |  | [optional]
**flow_file_summaries** | Option<[**Vec<crate::models::FlowFileSummaryDto>**](FlowFileSummaryDTO.md)> | The FlowFile summaries. The summaries will be populated once the request has completed. | [optional]
**source_running** | Option<**bool**> | Whether the source of the connection is running | [optional]
**destination_running** | Option<**bool**> | Whether the destination of the connection is running | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


