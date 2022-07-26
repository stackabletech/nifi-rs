# CountersDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregate_snapshot** | Option<[**crate::models::CountersSnapshotDto**](CountersSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeCountersSnapshotDto>**](NodeCountersSnapshotDTO.md)> | A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


