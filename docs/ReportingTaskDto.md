# ReportingTaskDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**name** | Option<**String**> | The name of the reporting task. | [optional]
**_type** | Option<**String**> | The fully qualified type of the reporting task. | [optional]
**bundle** | Option<[**crate::models::BundleDto**](BundleDTO.md)> |  | [optional]
**state** | Option<**String**> | The state of the reporting task. | [optional]
**comments** | Option<**String**> | The comments of the reporting task. | [optional]
**persists_state** | Option<**bool**> | Whether the reporting task persists state. | [optional]
**restricted** | Option<**bool**> | Whether the reporting task requires elevated privileges. | [optional]
**deprecated** | Option<**bool**> | Whether the reporting task has been deprecated. | [optional]
**multiple_versions_available** | Option<**bool**> | Whether the reporting task has multiple versions available. | [optional]
**scheduling_period** | Option<**String**> | The frequency with which to schedule the reporting task. The format of the value willd epend on the valud of the schedulingStrategy. | [optional]
**scheduling_strategy** | Option<**String**> | The scheduling strategy that determines how the schedulingPeriod value should be interpreted. | [optional]
**default_scheduling_period** | Option<**::std::collections::HashMap<String, String>**> | The default scheduling period for the different scheduling strategies. | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties of the reporting task. | [optional]
**descriptors** | Option<[**::std::collections::HashMap<String, crate::models::PropertyDescriptorDto>**](PropertyDescriptorDTO.md)> | The descriptors for the reporting tasks properties. | [optional]
**custom_ui_url** | Option<**String**> | The URL for the custom configuration UI for the reporting task. | [optional]
**annotation_data** | Option<**String**> | The annotation data for the repoting task. This is how the custom UI relays configuration to the reporting task. | [optional]
**validation_errors** | Option<**Vec<String>**> | Gets the validation errors from the reporting task. These validation errors represent the problems with the reporting task that must be resolved before it can be scheduled to run. | [optional]
**validation_status** | Option<**String**> | Indicates whether the Processor is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Processor is valid) | [optional]
**active_thread_count** | Option<**i32**> | The number of active threads for the reporting task. | [optional]
**extension_missing** | Option<**bool**> | Whether the underlying extension is missing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


