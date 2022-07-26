# ProcessGroupStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the Process Group | [optional]
**name** | Option<**String**> | The name of the Process Group | [optional]
**stats_last_refreshed** | Option<**String**> | The time the status for the process group was last refreshed. | [optional]
**aggregate_snapshot** | Option<[**crate::models::ProcessGroupStatusSnapshotDto**](ProcessGroupStatusSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeProcessGroupStatusSnapshotDto>**](NodeProcessGroupStatusSnapshotDTO.md)> | The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


