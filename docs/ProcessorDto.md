# ProcessorDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**name** | Option<**String**> | The name of the processor. | [optional]
**_type** | Option<**String**> | The type of the processor. | [optional]
**bundle** | Option<[**crate::models::BundleDto**](BundleDTO.md)> |  | [optional]
**state** | Option<**String**> | The state of the processor | [optional]
**style** | Option<**::std::collections::HashMap<String, String>**> | Styles for the processor (background-color : #eee). | [optional]
**relationships** | Option<[**Vec<crate::models::RelationshipDto>**](RelationshipDTO.md)> | The available relationships that the processor currently supports. | [optional]
**description** | Option<**String**> | The description of the processor. | [optional]
**supports_parallel_processing** | Option<**bool**> | Whether the processor supports parallel processing. | [optional]
**supports_event_driven** | Option<**bool**> | Whether the processor supports event driven scheduling. | [optional]
**supports_batching** | Option<**bool**> | Whether the processor supports batching. This makes the run duration settings available. | [optional]
**persists_state** | Option<**bool**> | Whether the processor persists state. | [optional]
**restricted** | Option<**bool**> | Whether the processor requires elevated privileges. | [optional]
**deprecated** | Option<**bool**> | Whether the processor has been deprecated. | [optional]
**execution_node_restricted** | Option<**bool**> | Indicates if the execution node of a processor is restricted to run only on the primary node | [optional]
**multiple_versions_available** | Option<**bool**> | Whether the processor has multiple versions available. | [optional]
**input_requirement** | Option<**String**> | The input requirement for this processor. | [optional]
**config** | Option<[**crate::models::ProcessorConfigDto**](ProcessorConfigDTO.md)> |  | [optional]
**validation_errors** | Option<**Vec<String>**> | The validation errors for the processor. These validation errors represent the problems with the processor that must be resolved before it can be started. | [optional]
**validation_status** | Option<**String**> | Indicates whether the Processor is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Processor is valid) | [optional]
**extension_missing** | Option<**bool**> | Whether the underlying extension is missing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


