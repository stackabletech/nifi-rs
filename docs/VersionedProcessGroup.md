# VersionedProcessGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**process_groups** | Option<[**Vec<crate::models::VersionedProcessGroup>**](VersionedProcessGroup.md)> | The child Process Groups | [optional]
**remote_process_groups** | Option<[**Vec<crate::models::VersionedRemoteProcessGroup>**](VersionedRemoteProcessGroup.md)> | The Remote Process Groups | [optional]
**processors** | Option<[**Vec<crate::models::VersionedProcessor>**](VersionedProcessor.md)> | The Processors | [optional]
**input_ports** | Option<[**Vec<crate::models::VersionedPort>**](VersionedPort.md)> | The Input Ports | [optional]
**output_ports** | Option<[**Vec<crate::models::VersionedPort>**](VersionedPort.md)> | The Output Ports | [optional]
**connections** | Option<[**Vec<crate::models::VersionedConnection>**](VersionedConnection.md)> | The Connections | [optional]
**labels** | Option<[**Vec<crate::models::VersionedLabel>**](VersionedLabel.md)> | The Labels | [optional]
**funnels** | Option<[**Vec<crate::models::VersionedFunnel>**](VersionedFunnel.md)> | The Funnels | [optional]
**controller_services** | Option<[**Vec<crate::models::VersionedControllerService>**](VersionedControllerService.md)> | The Controller Services | [optional]
**versioned_flow_coordinates** | Option<[**crate::models::VersionedFlowCoordinates**](VersionedFlowCoordinates.md)> |  | [optional]
**variables** | Option<**::std::collections::HashMap<String, String>**> | The Variables in the Variable Registry for this Process Group (not including any ancestor or descendant Process Groups) | [optional]
**parameter_context_name** | Option<**String**> | The name of the parameter context used by this process group | [optional]
**default_flow_file_expiration** | Option<**String**> | The default FlowFile Expiration for this Process Group. | [optional]
**default_back_pressure_object_threshold** | Option<**i64**> | Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied. | [optional]
**default_back_pressure_data_size_threshold** | Option<**String**> | Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied. | [optional]
**flow_file_concurrency** | Option<**String**> | The configured FlowFile Concurrency for the Process Group | [optional]
**flow_file_outbound_policy** | Option<**String**> | The FlowFile Outbound Policy for the Process Group | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


