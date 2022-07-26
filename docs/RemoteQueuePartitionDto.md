# RemoteQueuePartitionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_flow_file_count** | Option<**i32**> | Total number of FlowFiles owned by the Connection | [optional]
**total_byte_count** | Option<**i64**> | Total number of bytes that make up the content for the FlowFiles owned by this Connection | [optional]
**active_queue_flow_file_count** | Option<**i32**> | Total number of FlowFiles that exist in the Connection's Active Queue, immediately available to be offered up to a component | [optional]
**active_queue_byte_count** | Option<**i64**> | Total number of bytes that make up the content for the FlowFiles that are present in the Connection's Active Queue | [optional]
**swap_flow_file_count** | Option<**i32**> | The total number of FlowFiles that are swapped out for this Connection | [optional]
**swap_byte_count** | Option<**i64**> | Total number of bytes that make up the content for the FlowFiles that are swapped out to disk for the Connection | [optional]
**swap_files** | Option<**i32**> | The number of Swap Files that exist for this Connection | [optional]
**in_flight_flow_file_count** | Option<**i32**> | The number of In-Flight FlowFiles for this Connection. These are FlowFiles that belong to the connection but are currently being operated on by a Processor, Port, etc. | [optional]
**in_flight_byte_count** | Option<**i64**> | The number bytes that make up the content of the FlowFiles that are In-Flight | [optional]
**node_identifier** | Option<**String**> | The Node Identifier that this queue partition is sending to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


