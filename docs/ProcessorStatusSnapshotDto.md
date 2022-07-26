# ProcessorStatusSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the processor. | [optional]
**group_id** | Option<**String**> | The id of the parent process group to which the processor belongs. | [optional]
**name** | Option<**String**> | The name of the prcessor. | [optional]
**_type** | Option<**String**> | The type of the processor. | [optional]
**run_status** | Option<**String**> | The state of the processor. | [optional]
**execution_node** | Option<**String**> | Indicates the node where the process will execute. | [optional]
**bytes_read** | Option<**i64**> | The number of bytes read by this Processor in the last 5 mintues | [optional]
**bytes_written** | Option<**i64**> | The number of bytes written by this Processor in the last 5 minutes | [optional]
**read** | Option<**String**> | The number of bytes read in the last 5 minutes. | [optional]
**written** | Option<**String**> | The number of bytes written in the last 5 minutes. | [optional]
**flow_files_in** | Option<**i32**> | The number of FlowFiles that have been accepted in the last 5 minutes | [optional]
**bytes_in** | Option<**i64**> | The size of the FlowFiles that have been accepted in the last 5 minutes | [optional]
**input** | Option<**String**> | The count/size of flowfiles that have been accepted in the last 5 minutes. | [optional]
**flow_files_out** | Option<**i32**> | The number of FlowFiles transferred to a Connection in the last 5 minutes | [optional]
**bytes_out** | Option<**i64**> | The size of the FlowFiles transferred to a Connection in the last 5 minutes | [optional]
**output** | Option<**String**> | The count/size of flowfiles that have been processed in the last 5 minutes. | [optional]
**task_count** | Option<**i32**> | The number of times this Processor has run in the last 5 minutes | [optional]
**tasks_duration_nanos** | Option<**i64**> | The number of nanoseconds that this Processor has spent running in the last 5 minutes | [optional]
**tasks** | Option<**String**> | The total number of task this connectable has completed over the last 5 minutes. | [optional]
**tasks_duration** | Option<**String**> | The total duration of all tasks for this connectable over the last 5 minutes. | [optional]
**active_thread_count** | Option<**i32**> | The number of threads currently executing in the processor. | [optional]
**terminated_thread_count** | Option<**i32**> | The number of threads currently terminated for the processor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


