# RemoteProcessGroupDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**target_uri** | Option<**String**> | The target URI of the remote process group. If target uri is not set, but uris are set, then returns the first url in the urls. If neither target uri nor uris are set, then returns null. | [optional]
**target_uris** | Option<**String**> | The target URI of the remote process group. If target uris is not set but target uri is set, then returns a collection containing the single target uri. If neither target uris nor uris are set, then returns null. | [optional]
**target_secure** | Option<**bool**> | Whether the target is running securely. | [optional]
**name** | Option<**String**> | The name of the remote process group. | [optional]
**comments** | Option<**String**> | The comments for the remote process group. | [optional]
**communications_timeout** | Option<**String**> | The time period used for the timeout when communicating with the target. | [optional]
**yield_duration** | Option<**String**> | When yielding, this amount of time must elapse before the remote process group is scheduled again. | [optional]
**transport_protocol** | Option<**String**> |  | [optional]
**local_network_interface** | Option<**String**> | The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier. | [optional]
**proxy_host** | Option<**String**> |  | [optional]
**proxy_port** | Option<**i32**> |  | [optional]
**proxy_user** | Option<**String**> |  | [optional]
**proxy_password** | Option<**String**> |  | [optional]
**authorization_issues** | Option<**Vec<String>**> | Any remote authorization issues for the remote process group. | [optional]
**validation_errors** | Option<**Vec<String>**> | The validation errors for the remote process group. These validation errors represent the problems with the remote process group that must be resolved before it can transmit. | [optional]
**transmitting** | Option<**bool**> | Whether the remote process group is actively transmitting. | [optional]
**input_port_count** | Option<**i32**> | The number of remote input ports currently available on the target. | [optional]
**output_port_count** | Option<**i32**> | The number of remote output ports currently available on the target. | [optional]
**active_remote_input_port_count** | Option<**i32**> | The number of active remote input ports. | [optional]
**inactive_remote_input_port_count** | Option<**i32**> | The number of inactive remote input ports. | [optional]
**active_remote_output_port_count** | Option<**i32**> | The number of active remote output ports. | [optional]
**inactive_remote_output_port_count** | Option<**i32**> | The number of inactive remote output ports. | [optional]
**flow_refreshed** | Option<**String**> | The timestamp when this remote process group was last refreshed. | [optional]
**contents** | Option<[**crate::models::RemoteProcessGroupContentsDto**](RemoteProcessGroupContentsDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


