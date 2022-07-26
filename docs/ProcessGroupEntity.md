# ProcessGroupEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**revision** | Option<[**crate::models::RevisionDto**](RevisionDTO.md)> |  | [optional]
**id** | Option<**String**> | The id of the component. | [optional]
**uri** | Option<**String**> | The URI for futures requests to the component. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**permissions** | Option<[**crate::models::PermissionsDto**](PermissionsDTO.md)> |  | [optional]
**bulletins** | Option<[**Vec<crate::models::BulletinEntity>**](BulletinEntity.md)> | The bulletins for this component. | [optional]
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. | [optional]
**component** | Option<[**crate::models::ProcessGroupDto**](ProcessGroupDTO.md)> |  | [optional]
**status** | Option<[**crate::models::ProcessGroupStatusDto**](ProcessGroupStatusDTO.md)> |  | [optional]
**versioned_flow_snapshot** | Option<[**crate::models::VersionedFlowSnapshot**](VersionedFlowSnapshot.md)> |  | [optional]
**running_count** | Option<**i32**> | The number of running components in this process group. | [optional]
**stopped_count** | Option<**i32**> | The number of stopped components in the process group. | [optional]
**invalid_count** | Option<**i32**> | The number of invalid components in the process group. | [optional]
**disabled_count** | Option<**i32**> | The number of disabled components in the process group. | [optional]
**active_remote_port_count** | Option<**i32**> | The number of active remote ports in the process group. | [optional]
**inactive_remote_port_count** | Option<**i32**> | The number of inactive remote ports in the process group. | [optional]
**versioned_flow_state** | Option<**String**> | The current state of the Process Group, as it relates to the Versioned Flow | [optional]
**up_to_date_count** | Option<**i32**> | The number of up to date versioned process groups in the process group. | [optional]
**locally_modified_count** | Option<**i32**> | The number of locally modified versioned process groups in the process group. | [optional]
**stale_count** | Option<**i32**> | The number of stale versioned process groups in the process group. | [optional]
**locally_modified_and_stale_count** | Option<**i32**> | The number of locally modified and stale versioned process groups in the process group. | [optional]
**sync_failure_count** | Option<**i32**> | The number of versioned process groups in the process group that are unable to sync to a registry. | [optional]
**local_input_port_count** | Option<**i32**> | The number of local input ports in the process group. | [optional]
**local_output_port_count** | Option<**i32**> | The number of local output ports in the process group. | [optional]
**public_input_port_count** | Option<**i32**> | The number of public input ports in the process group. | [optional]
**public_output_port_count** | Option<**i32**> | The number of public output ports in the process group. | [optional]
**parameter_context** | Option<[**crate::models::ParameterContextReferenceEntity**](ParameterContextReferenceEntity.md)> |  | [optional]
**input_port_count** | Option<**i32**> | The number of input ports in the process group. | [optional]
**output_port_count** | Option<**i32**> | The number of output ports in the process group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


