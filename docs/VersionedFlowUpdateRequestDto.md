# VersionedFlowUpdateRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | The unique ID of this request. | [optional]
**process_group_id** | Option<**String**> | The unique ID of the Process Group being updated | [optional]
**uri** | Option<**String**> | The URI for future requests to this drop request. | [optional]
**last_updated** | Option<**String**> | The last time this request was updated. | [optional]
**complete** | Option<**bool**> | Whether or not this request has completed | [optional]
**failure_reason** | Option<**String**> | An explanation of why this request failed, or null if this request has not failed | [optional]
**percent_completed** | Option<**i32**> | The percentage complete for the request, between 0 and 100 | [optional]
**state** | Option<**String**> | The state of the request | [optional]
**version_control_information** | Option<[**crate::models::VersionControlInformationDto**](VersionControlInformationDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


