# ConnectionStatisticsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the connection | [optional]
**stats_last_refreshed** | Option<**String**> | The timestamp of when the stats were last refreshed | [optional]
**aggregate_snapshot** | Option<[**crate::models::ConnectionStatisticsSnapshotDto**](ConnectionStatisticsSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeConnectionStatisticsSnapshotDto>**](NodeConnectionStatisticsSnapshotDTO.md)> | A list of status snapshots for each node | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


