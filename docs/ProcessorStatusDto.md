# ProcessorStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**String**> | The unique ID of the process group that the Processor belongs to | [optional]
**id** | Option<**String**> | The unique ID of the Processor | [optional]
**name** | Option<**String**> | The name of the Processor | [optional]
**_type** | Option<**String**> | The type of the Processor | [optional]
**run_status** | Option<**String**> | The run status of the Processor | [optional]
**stats_last_refreshed** | Option<**String**> | The timestamp of when the stats were last refreshed | [optional]
**aggregate_snapshot** | Option<[**crate::models::ProcessorStatusSnapshotDto**](ProcessorStatusSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeProcessorStatusSnapshotDto>**](NodeProcessorStatusSnapshotDTO.md)> | A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


