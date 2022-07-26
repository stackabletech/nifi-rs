# PortStatusSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the port. | [optional]
**group_id** | Option<**String**> | The id of the parent process group of the port. | [optional]
**name** | Option<**String**> | The name of the port. | [optional]
**active_thread_count** | Option<**i32**> | The active thread count for the port. | [optional]
**flow_files_in** | Option<**i32**> | The number of FlowFiles that have been accepted in the last 5 minutes. | [optional]
**bytes_in** | Option<**i64**> | The size of hte FlowFiles that have been accepted in the last 5 minutes. | [optional]
**input** | Option<**String**> | The count/size of flowfiles that have been accepted in the last 5 minutes. | [optional]
**flow_files_out** | Option<**i32**> | The number of FlowFiles that have been processed in the last 5 minutes. | [optional]
**bytes_out** | Option<**i64**> | The number of bytes that have been processed in the last 5 minutes. | [optional]
**output** | Option<**String**> | The count/size of flowfiles that have been processed in the last 5 minutes. | [optional]
**transmitting** | Option<**bool**> | Whether the port has incoming or outgoing connections to a remote NiFi. | [optional]
**run_status** | Option<**String**> | The run status of the port. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


