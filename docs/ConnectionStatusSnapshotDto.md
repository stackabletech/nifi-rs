# ConnectionStatusSnapshotDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the connection. | [optional]
**group_id** | Option<**String**> | The id of the process group the connection belongs to. | [optional]
**name** | Option<**String**> | The name of the connection. | [optional]
**source_id** | Option<**String**> | The id of the source of the connection. | [optional]
**source_name** | Option<**String**> | The name of the source of the connection. | [optional]
**destination_id** | Option<**String**> | The id of the destination of the connection. | [optional]
**destination_name** | Option<**String**> | The name of the destination of the connection. | [optional]
**predictions** | Option<[**crate::models::ConnectionStatusPredictionsSnapshotDto**](ConnectionStatusPredictionsSnapshotDTO.md)> |  | [optional]
**flow_files_in** | Option<**i32**> | The number of FlowFiles that have come into the connection in the last 5 minutes. | [optional]
**bytes_in** | Option<**i64**> | The size of the FlowFiles that have come into the connection in the last 5 minutes. | [optional]
**input** | Option<**String**> | The input count/size for the connection in the last 5 minutes, pretty printed. | [optional]
**flow_files_out** | Option<**i32**> | The number of FlowFiles that have left the connection in the last 5 minutes. | [optional]
**bytes_out** | Option<**i64**> | The number of bytes that have left the connection in the last 5 minutes. | [optional]
**output** | Option<**String**> | The output count/sie for the connection in the last 5 minutes, pretty printed. | [optional]
**flow_files_queued** | Option<**i32**> | The number of FlowFiles that are currently queued in the connection. | [optional]
**bytes_queued** | Option<**i64**> | The size of the FlowFiles that are currently queued in the connection. | [optional]
**queued** | Option<**String**> | The total count and size of queued flowfiles formatted. | [optional]
**queued_size** | Option<**String**> | The total size of flowfiles that are queued formatted. | [optional]
**queued_count** | Option<**String**> | The number of flowfiles that are queued, pretty printed. | [optional]
**percent_use_count** | Option<**i32**> | Connection percent use regarding queued flow files count and backpressure threshold if configured. | [optional]
**percent_use_bytes** | Option<**i32**> | Connection percent use regarding queued flow files size and backpressure threshold if configured. | [optional]
**flow_file_availability** | Option<**String**> | The availability of FlowFiles in this connection | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


