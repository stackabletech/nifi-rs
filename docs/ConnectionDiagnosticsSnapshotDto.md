# ConnectionDiagnosticsSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_flow_file_count** | Option<**i32**> | Total number of FlowFiles owned by the Connection | [optional]
**total_byte_count** | Option<**i64**> | Total number of bytes that make up the content for the FlowFiles owned by this Connection | [optional]
**node_identifier** | Option<**String**> | The Node Identifier that this information pertains to | [optional]
**local_queue_partition** | Option<[**crate::models::LocalQueuePartitionDto**](LocalQueuePartitionDTO.md)> |  | [optional]
**remote_queue_partitions** | Option<[**Vec<crate::models::RemoteQueuePartitionDto>**](RemoteQueuePartitionDTO.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


