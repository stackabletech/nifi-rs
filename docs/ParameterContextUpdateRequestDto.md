# ParameterContextUpdateRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | The ID of the request | [optional]
**uri** | Option<**String**> | The URI for the request | [optional]
**submission_time** | Option<**String**> | The timestamp of when the request was submitted | [optional]
**last_updated** | Option<**String**> | The timestamp of when the request was last updated | [optional]
**complete** | Option<**bool**> | Whether or not the request is completed | [optional]
**failure_reason** | Option<**String**> | The reason for the request failing, or null if the request has not failed | [optional]
**percent_completed** | Option<**i32**> | A value between 0 and 100 (inclusive) indicating how close the request is to completion | [optional]
**state** | Option<**String**> | A description of the current state of the request | [optional]
**update_steps** | Option<[**Vec<crate::models::ParameterContextUpdateStepDto>**](ParameterContextUpdateStepDTO.md)> | The steps that are required in order to complete the request, along with the status of each | [optional]
**parameter_context** | Option<[**crate::models::ParameterContextDto**](ParameterContextDTO.md)> |  | [optional]
**referencing_components** | Option<[**Vec<crate::models::AffectedComponentEntity>**](AffectedComponentEntity.md)> | The components that are referenced by the update. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


