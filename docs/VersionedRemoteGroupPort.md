# VersionedRemoteGroupPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**remote_group_id** | Option<**String**> | The id of the remote process group that the port resides in. | [optional]
**concurrently_schedulable_task_count** | Option<**i32**> | The number of task that may transmit flowfiles to the target port concurrently. | [optional]
**use_compression** | Option<**bool**> | Whether the flowfiles are compressed when sent to the target port. | [optional]
**batch_size** | Option<[**crate::models::BatchSize**](BatchSize.md)> |  | [optional]
**component_type** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> | The ID of the port on the target NiFi instance | [optional]
**scheduled_state** | Option<**String**> | The scheduled state of the component | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


