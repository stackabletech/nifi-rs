# VersionedPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**_type** | Option<**String**> | The type of port. | [optional]
**concurrently_schedulable_task_count** | Option<**i32**> | The number of tasks that should be concurrently scheduled for the port. | [optional]
**scheduled_state** | Option<**String**> | The scheduled state of the component | [optional]
**allow_remote_access** | Option<**bool**> | Whether or not this port allows remote access for site-to-site | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


