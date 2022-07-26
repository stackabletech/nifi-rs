# PortStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the port. | [optional]
**group_id** | Option<**String**> | The id of the parent process group of the port. | [optional]
**name** | Option<**String**> | The name of the port. | [optional]
**transmitting** | Option<**bool**> | Whether the port has incoming or outgoing connections to a remote NiFi. | [optional]
**run_status** | Option<**String**> | The run status of the port. | [optional]
**stats_last_refreshed** | Option<**String**> | The time the status for the process group was last refreshed. | [optional]
**aggregate_snapshot** | Option<[**crate::models::PortStatusSnapshotDto**](PortStatusSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodePortStatusSnapshotDto>**](NodePortStatusSnapshotDTO.md)> | A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


