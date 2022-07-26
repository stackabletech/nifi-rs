# ProcessorConfigDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties for the processor. Properties whose value is not set will only contain the property name. | [optional]
**descriptors** | Option<[**::std::collections::HashMap<String, crate::models::PropertyDescriptorDto>**](PropertyDescriptorDTO.md)> | Descriptors for the processor's properties. | [optional]
**scheduling_period** | Option<**String**> | The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy. | [optional]
**scheduling_strategy** | Option<**String**> | Indcates whether the prcessor should be scheduled to run in event or timer driven mode. | [optional]
**execution_node** | Option<**String**> | Indicates the node where the process will execute. | [optional]
**penalty_duration** | Option<**String**> | The amount of time that is used when the process penalizes a flowfile. | [optional]
**yield_duration** | Option<**String**> | The amount of time that must elapse before this processor is scheduled again after yielding. | [optional]
**bulletin_level** | Option<**String**> | The level at which the processor will report bulletins. | [optional]
**run_duration_millis** | Option<**i64**> | The run duration for the processor in milliseconds. | [optional]
**concurrently_schedulable_task_count** | Option<**i32**> | The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored. | [optional]
**auto_terminated_relationships** | Option<**Vec<String>**> | The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated. | [optional]
**comments** | Option<**String**> | The comments for the processor. | [optional]
**custom_ui_url** | Option<**String**> | The URL for the processor's custom configuration UI if applicable. | [optional]
**loss_tolerant** | Option<**bool**> | Whether the processor is loss tolerant. | [optional]
**annotation_data** | Option<**String**> | The annotation data for the processor used to relay configuration between a custom UI and the procesosr. | [optional]
**default_concurrent_tasks** | Option<**::std::collections::HashMap<String, String>**> | Maps default values for concurrent tasks for each applicable scheduling strategy. | [optional]
**default_scheduling_period** | Option<**::std::collections::HashMap<String, String>**> | Maps default values for scheduling period for each applicable scheduling strategy. | [optional]
**retry_count** | Option<**i32**> | Overall number of retries. | [optional]
**retried_relationships** | Option<**Vec<String>**> | All the relationships should be retried. | [optional]
**backoff_mechanism** | Option<**String**> | Determines whether the FlowFile should be penalized or the processor should be yielded between retries. | [optional]
**max_backoff_period** | Option<**String**> | Maximum amount of time to be waited during a retry period. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


