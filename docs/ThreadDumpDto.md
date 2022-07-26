# ThreadDumpDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**String**> | The ID of the node in the cluster | [optional]
**node_address** | Option<**String**> | The address of the node in the cluster | [optional]
**api_port** | Option<**i32**> | The port the node is listening for API requests. | [optional]
**stack_trace** | Option<**String**> | The stack trace for the thread | [optional]
**thread_name** | Option<**String**> | The name of the thread | [optional]
**thread_active_millis** | Option<**i64**> | The number of milliseconds that the thread has been executing in the Processor | [optional]
**task_terminated** | Option<**bool**> | Indicates whether or not the user has requested that the task be terminated. If this is true, it may indicate that the thread is in a state where it will continue running indefinitely without returning. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


