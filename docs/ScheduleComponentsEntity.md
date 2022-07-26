# ScheduleComponentsEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the ProcessGroup | [optional]
**state** | Option<**String**> | The desired state of the descendant components | [optional]
**components** | Option<[**::std::collections::HashMap<String, crate::models::RevisionDto>**](RevisionDTO.md)> | Optional components to schedule. If not specified, all authorized descendant components will be used. | [optional]
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


