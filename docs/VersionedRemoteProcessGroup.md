# VersionedRemoteProcessGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**target_uri** | Option<**String**> | [DEPRECATED] The target URI of the remote process group. If target uri is not set, but uris are set, then returns the first uri in the uris. If neither target uri nor uris are set, then returns null. | [optional]
**target_uris** | Option<**String**> | The target URIs of the remote process group. If target uris is not set but target uri is set, then returns the single target uri. If neither target uris nor target uri is set, then returns null. | [optional]
**communications_timeout** | Option<**String**> | The time period used for the timeout when communicating with the target. | [optional]
**yield_duration** | Option<**String**> | When yielding, this amount of time must elapse before the remote process group is scheduled again. | [optional]
**transport_protocol** | Option<**String**> | The Transport Protocol that is used for Site-to-Site communications | [optional]
**local_network_interface** | Option<**String**> | The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier. | [optional]
**proxy_host** | Option<**String**> |  | [optional]
**proxy_port** | Option<**i32**> |  | [optional]
**proxy_user** | Option<**String**> |  | [optional]
**input_ports** | Option<[**Vec<crate::models::VersionedRemoteGroupPort>**](VersionedRemoteGroupPort.md)> | A Set of Input Ports that can be connected to, in order to send data to the remote NiFi instance | [optional]
**output_ports** | Option<[**Vec<crate::models::VersionedRemoteGroupPort>**](VersionedRemoteGroupPort.md)> | A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


