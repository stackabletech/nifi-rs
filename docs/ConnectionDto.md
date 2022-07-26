# ConnectionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the component. | [optional]
**versioned_component_id** | Option<**String**> | The ID of the corresponding component that is under version control | [optional]
**parent_group_id** | Option<**String**> | The id of parent process group of this component if applicable. | [optional]
**position** | Option<[**crate::models::PositionDto**](PositionDTO.md)> |  | [optional]
**source** | Option<[**crate::models::ConnectableDto**](ConnectableDTO.md)> |  | [optional]
**destination** | Option<[**crate::models::ConnectableDto**](ConnectableDTO.md)> |  | [optional]
**name** | Option<**String**> | The name of the connection. | [optional]
**label_index** | Option<**i32**> | The index of the bend point where to place the connection label. | [optional]
**getz_index** | Option<**i64**> | The z index of the connection. | [optional]
**selected_relationships** | Option<**Vec<String>**> | The selected relationship that comprise the connection. | [optional]
**available_relationships** | Option<**Vec<String>**> | The relationships that the source of the connection currently supports. | [optional]
**back_pressure_object_threshold** | Option<**i64**> | The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue. | [optional]
**back_pressure_data_size_threshold** | Option<**String**> | The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue. | [optional]
**flow_file_expiration** | Option<**String**> | The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it. | [optional]
**prioritizers** | Option<**Vec<String>**> | The comparators used to prioritize the queue. | [optional]
**bends** | Option<[**Vec<crate::models::PositionDto>**](PositionDTO.md)> | The bend points on the connection. | [optional]
**load_balance_strategy** | Option<**String**> | How to load balance the data in this Connection across the nodes in the cluster. | [optional]
**load_balance_partition_attribute** | Option<**String**> | The FlowFile Attribute to use for determining which node a FlowFile will go to if the Load Balancing Strategy is set to PARTITION_BY_ATTRIBUTE | [optional]
**load_balance_compression** | Option<**String**> | Whether or not data should be compressed when being transferred between nodes in the cluster. | [optional]
**load_balance_status** | Option<**String**> | The current status of the Connection's Load Balancing Activities. Status can indicate that Load Balancing is not configured for the connection, that Load Balancing is configured but inactive (not currently transferring data to another node), or that Load Balancing is configured and actively transferring data to another node. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


