# PortDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**name** | Option<**String**> | The name of the port. | [optional]
**comments** | Option<**String**> | The comments for the port. | [optional]
**state** | Option<**String**> | The state of the port. | [optional]
**_type** | Option<**String**> | The type of port. | [optional]
**transmitting** | Option<**bool**> | Whether the port has incoming or output connections to a remote NiFi. This is only applicable when the port is allowed to be accessed remotely. | [optional]
**concurrently_schedulable_task_count** | Option<**i32**> | The number of tasks that should be concurrently scheduled for the port. | [optional]
**user_access_control** | Option<**Vec<String>**> | The users that are allowed to access the port. | [optional]
**group_access_control** | Option<**Vec<String>**> | The user groups that are allowed to access the port. | [optional]
**allow_remote_access** | Option<**bool**> | Whether this port can be accessed remotely via Site-to-Site protocol. | [optional]
**validation_errors** | Option<**Vec<String>**> | Gets the validation errors from this port. These validation errors represent the problems with the port that must be resolved before it can be started. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


