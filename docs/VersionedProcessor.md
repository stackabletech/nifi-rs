# VersionedProcessor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**_type** | Option<**String**> | The type of the extension component | [optional]
**bundle** | Option<[**crate::models::Bundle**](Bundle.md)> |  | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties for the component. Properties whose value is not set will only contain the property name. | [optional]
**property_descriptors** | Option<[**::std::collections::HashMap<String, crate::models::VersionedPropertyDescriptor>**](VersionedPropertyDescriptor.md)> | The property descriptors for the component. | [optional]
**style** | Option<**::std::collections::HashMap<String, String>**> | Stylistic data for rendering in a UI | [optional]
**annotation_data** | Option<**String**> | The annotation data for the processor used to relay configuration between a custom UI and the procesosr. | [optional]
**scheduling_period** | Option<**String**> | The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy. | [optional]
**scheduling_strategy** | Option<**String**> | Indicates whether the processor should be scheduled to run in event or timer driven mode. | [optional]
**execution_node** | Option<**String**> | Indicates the node where the process will execute. | [optional]
**penalty_duration** | Option<**String**> | The amout of time that is used when the process penalizes a flowfile. | [optional]
**yield_duration** | Option<**String**> | The amount of time that must elapse before this processor is scheduled again after yielding. | [optional]
**bulletin_level** | Option<**String**> | The level at which the processor will report bulletins. | [optional]
**run_duration_millis** | Option<**i64**> | The run duration for the processor in milliseconds. | [optional]
**concurrently_schedulable_task_count** | Option<**i32**> | The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored. | [optional]
**auto_terminated_relationships** | Option<**Vec<String>**> | The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated. | [optional]
**scheduled_state** | Option<**String**> | The scheduled state of the component | [optional]
**retry_count** | Option<**i32**> | Overall number of retries. | [optional]
**retried_relationships** | Option<**Vec<String>**> | All the relationships should be retried. | [optional]
**backoff_mechanism** | Option<**String**> | Determines whether the FlowFile should be penalized or the processor should be yielded between retries. | [optional]
**max_backoff_period** | Option<**String**> | Maximum amount of time to be waited during a retry period. | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


