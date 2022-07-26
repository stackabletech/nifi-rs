# RemoteProcessGroupPortDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the port. | [optional]
**target_id** | Option<**String**> | The id of the target port. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**group_id** | Option<**String**> | The id of the remote process group that the port resides in. | [optional]
**name** | Option<**String**> | The name of the target port. | [optional]
**comments** | Option<**String**> | The comments as configured on the target port. | [optional]
**concurrently_schedulable_task_count** | Option<**i32**> | The number of task that may transmit flowfiles to the target port concurrently. | [optional]
**transmitting** | Option<**bool**> | Whether the remote port is configured for transmission. | [optional]
**use_compression** | Option<**bool**> | Whether the flowfiles are compressed when sent to the target port. | [optional]
**exists** | Option<**bool**> | Whether the target port exists. | [optional]
**target_running** | Option<**bool**> | Whether the target port is running. | [optional]
**connected** | Option<**bool**> | Whether the port has either an incoming or outgoing connection. | [optional]
**batch_settings** | Option<[**crate::models::BatchSettingsDto**](BatchSettingsDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


