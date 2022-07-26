# RemoteProcessGroupStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**String**> | The unique ID of the process group that the Processor belongs to | [optional]
**id** | Option<**String**> | The unique ID of the Processor | [optional]
**name** | Option<**String**> | The name of the remote process group. | [optional]
**target_uri** | Option<**String**> | The URI of the target system. | [optional]
**transmission_status** | Option<**String**> | The transmission status of the remote process group. | [optional]
**stats_last_refreshed** | Option<**String**> | The time the status for the process group was last refreshed. | [optional]
**validation_status** | Option<**String**> | Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) | [optional]
**aggregate_snapshot** | Option<[**crate::models::RemoteProcessGroupStatusSnapshotDto**](RemoteProcessGroupStatusSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeRemoteProcessGroupStatusSnapshotDto>**](NodeRemoteProcessGroupStatusSnapshotDTO.md)> | A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


