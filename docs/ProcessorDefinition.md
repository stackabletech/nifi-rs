# ProcessorDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group** | Option<**String**> | The group name of the bundle that provides the referenced type. | [optional]
**artifact** | Option<**String**> | The artifact name of the bundle that provides the referenced type. | [optional]
**version** | Option<**String**> | The version of the bundle that provides the referenced type. | [optional]
**_type** | **String** | The fully-qualified class type | 
**type_description** | Option<**String**> | The description of the type. | [optional]
**build_info** | Option<[**crate::models::BuildInfo**](BuildInfo.md)> |  | [optional]
**provided_api_implementations** | Option<[**Vec<crate::models::DefinedType>**](DefinedType.md)> | If this type represents a provider for an interface, this lists the APIs it implements | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with this type | [optional]
**deprecated** | Option<**bool**> | Whether or not the component has been deprecated | [optional]
**deprecation_reason** | Option<**String**> | If this component has been deprecated, this optional field can be used to provide an explanation | [optional]
**property_descriptors** | Option<[**::std::collections::HashMap<String, crate::models::PropertyDescriptor>**](PropertyDescriptor.md)> | Descriptions of configuration properties applicable to this component. | [optional]
**supports_dynamic_properties** | Option<**bool**> | Whether or not this component makes use of dynamic (user-set) properties. | [optional]
**input_requirement** | Option<**String**> | Any input requirements this processor has. | [optional]
**supported_relationships** | Option<[**Vec<crate::models::Relationship>**](Relationship.md)> | The supported relationships for this processor. | [optional]
**supports_dynamic_relationships** | Option<**bool**> | Whether or not this processor supports dynamic relationships. | [optional]
**trigger_serially** | Option<**bool**> | Whether or not this processor should be triggered serially (i.e. no concurrent execution). | [optional]
**trigger_when_empty** | Option<**bool**> | Whether or not this processor should be triggered when incoming queues are empty. | [optional]
**trigger_when_any_destination_available** | Option<**bool**> | Whether or not this processor should be triggered when any destination queue has room. | [optional]
**supports_batching** | Option<**bool**> | Whether or not this processor supports batching. If a Processor uses this annotation, it allows the Framework to batch calls to session commits, as well as allowing the Framework to return the same session multiple times. | [optional]
**supports_event_driven** | Option<**bool**> | Whether or not this processor supports event driven scheduling. Indicates to the framework that the Processor is eligible to be scheduled to run based on the occurrence of an \"Event\" (e.g., when a FlowFile is enqueued in an incoming Connection), rather than being triggered periodically. | [optional]
**primary_node_only** | Option<**bool**> | Whether or not this processor should be scheduled only on the primary node in a cluster. | [optional]
**side_effect_free** | Option<**bool**> | Whether or not this processor is considered side-effect free. Side-effect free indicate that the processor's operations on FlowFiles can be safely repeated across process sessions. | [optional]
**supported_scheduling_strategies** | Option<**Vec<String>**> | The supported scheduling strategies, such as TIME_DRIVER, CRON, or EVENT_DRIVEN. | [optional]
**default_scheduling_strategy** | Option<**String**> | The default scheduling strategy for the processor. | [optional]
**default_concurrent_tasks_by_scheduling_strategy** | Option<**::std::collections::HashMap<String, i32>**> | The default concurrent tasks for each scheduling strategy. | [optional]
**default_scheduling_period_by_scheduling_strategy** | Option<**::std::collections::HashMap<String, String>**> | The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as \"30 sec\". | [optional]
**default_penalty_duration** | Option<**String**> | The default penalty duration as a time period, such as \"30 sec\". | [optional]
**default_yield_duration** | Option<**String**> | The default yield duration as a time period, such as \"1 sec\". | [optional]
**default_bulletin_level** | Option<**String**> | The default bulletin level, such as WARN, INFO, DEBUG, etc. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


