# ProcessGroupStatusSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the process group. | [optional]
**name** | Option<**String**> | The name of this process group. | [optional]
**connection_status_snapshots** | Option<[**Vec<crate::models::ConnectionStatusSnapshotEntity>**](ConnectionStatusSnapshotEntity.md)> | The status of all connections in the process group. | [optional]
**processor_status_snapshots** | Option<[**Vec<crate::models::ProcessorStatusSnapshotEntity>**](ProcessorStatusSnapshotEntity.md)> | The status of all processors in the process group. | [optional]
**process_group_status_snapshots** | Option<[**Vec<crate::models::ProcessGroupStatusSnapshotEntity>**](ProcessGroupStatusSnapshotEntity.md)> | The status of all process groups in the process group. | [optional]
**remote_process_group_status_snapshots** | Option<[**Vec<crate::models::RemoteProcessGroupStatusSnapshotEntity>**](RemoteProcessGroupStatusSnapshotEntity.md)> | The status of all remote process groups in the process group. | [optional]
**input_port_status_snapshots** | Option<[**Vec<crate::models::PortStatusSnapshotEntity>**](PortStatusSnapshotEntity.md)> | The status of all input ports in the process group. | [optional]
**output_port_status_snapshots** | Option<[**Vec<crate::models::PortStatusSnapshotEntity>**](PortStatusSnapshotEntity.md)> | The status of all output ports in the process group. | [optional]
**versioned_flow_state** | Option<**String**> | The current state of the Process Group, as it relates to the Versioned Flow | [optional]
**flow_files_in** | Option<**i32**> | The number of FlowFiles that have come into this ProcessGroup in the last 5 minutes | [optional]
**bytes_in** | Option<**i64**> | The number of bytes that have come into this ProcessGroup in the last 5 minutes | [optional]
**input** | Option<**String**> | The input count/size for the process group in the last 5 minutes (pretty printed). | [optional]
**flow_files_queued** | Option<**i32**> | The number of FlowFiles that are queued up in this ProcessGroup right now | [optional]
**bytes_queued** | Option<**i64**> | The number of bytes that are queued up in this ProcessGroup right now | [optional]
**queued** | Option<**String**> | The count/size that is queued in the the process group. | [optional]
**queued_count** | Option<**String**> | The count that is queued for the process group. | [optional]
**queued_size** | Option<**String**> | The size that is queued for the process group. | [optional]
**bytes_read** | Option<**i64**> | The number of bytes read by components in this ProcessGroup in the last 5 minutes | [optional]
**read** | Option<**String**> | The number of bytes read in the last 5 minutes. | [optional]
**bytes_written** | Option<**i64**> | The number of bytes written by components in this ProcessGroup in the last 5 minutes | [optional]
**written** | Option<**String**> | The number of bytes written in the last 5 minutes. | [optional]
**flow_files_out** | Option<**i32**> | The number of FlowFiles transferred out of this ProcessGroup in the last 5 minutes | [optional]
**bytes_out** | Option<**i64**> | The number of bytes transferred out of this ProcessGroup in the last 5 minutes | [optional]
**output** | Option<**String**> | The output count/size for the process group in the last 5 minutes. | [optional]
**flow_files_transferred** | Option<**i32**> | The number of FlowFiles transferred in this ProcessGroup in the last 5 minutes | [optional]
**bytes_transferred** | Option<**i64**> | The number of bytes transferred in this ProcessGroup in the last 5 minutes | [optional]
**transferred** | Option<**String**> | The count/size transferred to/from queues in the process group in the last 5 minutes. | [optional]
**bytes_received** | Option<**i64**> | The number of bytes received from external sources by components within this ProcessGroup in the last 5 minutes | [optional]
**flow_files_received** | Option<**i32**> | The number of FlowFiles received from external sources by components within this ProcessGroup in the last 5 minutes | [optional]
**received** | Option<**String**> | The count/size sent to the process group in the last 5 minutes. | [optional]
**bytes_sent** | Option<**i64**> | The number of bytes sent to an external sink by components within this ProcessGroup in the last 5 minutes | [optional]
**flow_files_sent** | Option<**i32**> | The number of FlowFiles sent to an external sink by components within this ProcessGroup in the last 5 minutes | [optional]
**sent** | Option<**String**> | The count/size sent from this process group in the last 5 minutes. | [optional]
**active_thread_count** | Option<**i32**> | The active thread count for this process group. | [optional]
**terminated_thread_count** | Option<**i32**> | The number of threads currently terminated for the process group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


