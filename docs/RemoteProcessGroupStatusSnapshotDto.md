# RemoteProcessGroupStatusSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the remote process group. | [optional]
**group_id** | Option<**String**> | The id of the parent process group the remote process group resides in. | [optional]
**name** | Option<**String**> | The name of the remote process group. | [optional]
**target_uri** | Option<**String**> | The URI of the target system. | [optional]
**transmission_status** | Option<**String**> | The transmission status of the remote process group. | [optional]
**active_thread_count** | Option<**i32**> | The number of active threads for the remote process group. | [optional]
**flow_files_sent** | Option<**i32**> | The number of FlowFiles sent to the remote process group in the last 5 minutes. | [optional]
**bytes_sent** | Option<**i64**> | The size of the FlowFiles sent to the remote process group in the last 5 minutes. | [optional]
**sent** | Option<**String**> | The count/size of the flowfiles sent to the remote process group in the last 5 minutes. | [optional]
**flow_files_received** | Option<**i32**> | The number of FlowFiles received from the remote process group in the last 5 minutes. | [optional]
**bytes_received** | Option<**i64**> | The size of the FlowFiles received from the remote process group in the last 5 minutes. | [optional]
**received** | Option<**String**> | The count/size of the flowfiles received from the remote process group in the last 5 minutes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


