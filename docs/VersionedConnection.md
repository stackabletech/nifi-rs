# VersionedConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier** | Option<**String**> | The component's unique identifier | [optional]
**instance_identifier** | Option<**String**> | The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component | [optional]
**name** | Option<**String**> | The component's name | [optional]
**comments** | Option<**String**> | The user-supplied comments for the component | [optional]
**position** | Option<[**crate::models::Position**](Position.md)> |  | [optional]
**source** | Option<[**crate::models::ConnectableComponent**](ConnectableComponent.md)> |  | [optional]
**destination** | Option<[**crate::models::ConnectableComponent**](ConnectableComponent.md)> |  | [optional]
**label_index** | Option<**i32**> | The index of the bend point where to place the connection label. | [optional]
**z_index** | Option<**i64**> | The z index of the connection. | [optional]
**selected_relationships** | Option<**Vec<String>**> | The selected relationship that comprise the connection. | [optional]
**back_pressure_object_threshold** | Option<**i64**> | The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue. | [optional]
**back_pressure_data_size_threshold** | Option<**String**> | The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue. | [optional]
**flow_file_expiration** | Option<**String**> | The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it. | [optional]
**prioritizers** | Option<**Vec<String>**> | The comparators used to prioritize the queue. | [optional]
**bends** | Option<[**Vec<crate::models::Position>**](Position.md)> | The bend points on the connection. | [optional]
**load_balance_strategy** | Option<**String**> | The Strategy to use for load balancing data across the cluster, or null, if no Load Balance Strategy has been specified. | [optional]
**partitioning_attribute** | Option<**String**> | The attribute to use for partitioning data as it is load balanced across the cluster. If the Load Balance Strategy is configured to use PARTITION_BY_ATTRIBUTE, the value returned by this method is the name of the FlowFile Attribute that will be used to determine which node in the cluster should receive a given FlowFile. If the Load Balance Strategy is unset or is set to any other value, the Partitioning Attribute has no effect. | [optional]
**load_balance_compression** | Option<**String**> | Whether or not compression should be used when transferring FlowFiles between nodes | [optional]
**component_type** | Option<**String**> |  | [optional]
**group_identifier** | Option<**String**> | The ID of the Process Group that this component belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


