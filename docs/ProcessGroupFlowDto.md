# ProcessGroupFlowDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**uri** | Option<**String**> | The URI for futures requests to the component. | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**parameter_context** | Option<[**crate::models::ParameterContextReferenceEntity**](ParameterContextReferenceEntity.md)> |  | [optional]
**breadcrumb** | Option<[**crate::models::FlowBreadcrumbEntity**](FlowBreadcrumbEntity.md)> |  | [optional]
**flow** | Option<[**crate::models::FlowDto**](FlowDTO.md)> |  | [optional]
**last_refreshed** | Option<**String**> | The time the flow for the process group was last refreshed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


