# ProcessGroupDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**name** | Option<**String**> | The name of the process group. | [optional]
**comments** | Option<**String**> | The comments for the process group. | [optional]
**variables** | Option<**::std::collections::HashMap<String, String>**> | The variables that are configured for the Process Group. Note that this map contains only those variables that are defined on this Process Group and not any variables that are defined in the parent Process Group, etc. I.e., this Map will not contain all variables that are accessible by components in this Process Group by rather only the variables that are defined for this Process Group itself. | [optional]
**version_control_information** | Option<[**crate::models::VersionControlInformationDto**](VersionControlInformationDTO.md)> |  | [optional]
**parameter_context** | Option<[**crate::models::ParameterContextReferenceEntity**](ParameterContextReferenceEntity.md)> |  | [optional]
**flowfile_concurrency** | Option<**String**> | The FlowFile Concurrency for this Process Group. | [optional]
**flowfile_outbound_policy** | Option<**String**> | The Outbound Policy that is used for determining how FlowFiles should be transferred out of the Process Group. | [optional]
**default_flow_file_expiration** | Option<**String**> | The default FlowFile Expiration for this Process Group. | [optional]
**default_back_pressure_object_threshold** | Option<**i64**> | Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied. | [optional]
**default_back_pressure_data_size_threshold** | Option<**String**> | Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied. | [optional]
**running_count** | Option<**i32**> | The number of running components in this process group. | [optional]
**stopped_count** | Option<**i32**> | The number of stopped components in the process group. | [optional]
**invalid_count** | Option<**i32**> | The number of invalid components in the process group. | [optional]
**disabled_count** | Option<**i32**> | The number of disabled components in the process group. | [optional]
**active_remote_port_count** | Option<**i32**> | The number of active remote ports in the process group. | [optional]
**inactive_remote_port_count** | Option<**i32**> | The number of inactive remote ports in the process group. | [optional]
**up_to_date_count** | Option<**i32**> | The number of up to date versioned process groups in the process group. | [optional]
**locally_modified_count** | Option<**i32**> | The number of locally modified versioned process groups in the process group. | [optional]
**stale_count** | Option<**i32**> | The number of stale versioned process groups in the process group. | [optional]
**locally_modified_and_stale_count** | Option<**i32**> | The number of locally modified and stale versioned process groups in the process group. | [optional]
**sync_failure_count** | Option<**i32**> | The number of versioned process groups in the process group that are unable to sync to a registry. | [optional]
**local_input_port_count** | Option<**i32**> | The number of local input ports in the process group. | [optional]
**local_output_port_count** | Option<**i32**> | The number of local output ports in the process group. | [optional]
**public_input_port_count** | Option<**i32**> | The number of public input ports in the process group. | [optional]
**public_output_port_count** | Option<**i32**> | The number of public output ports in the process group. | [optional]
**contents** | Option<[**crate::models::FlowSnippetDto**](FlowSnippetDTO.md)> |  | [optional]
**input_port_count** | Option<**i32**> | The number of input ports in the process group. | [optional]
**output_port_count** | Option<**i32**> | The number of output ports in the process group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


