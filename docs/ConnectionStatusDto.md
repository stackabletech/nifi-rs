# ConnectionStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the connection | [optional]
**group_id** | Option<**String**> | The ID of the Process Group that the connection belongs to | [optional]
**name** | Option<**String**> | The name of the connection | [optional]
**stats_last_refreshed** | Option<**String**> | The timestamp of when the stats were last refreshed | [optional]
**source_id** | Option<**String**> | The ID of the source component | [optional]
**source_name** | Option<**String**> | The name of the source component | [optional]
**destination_id** | Option<**String**> | The ID of the destination component | [optional]
**destination_name** | Option<**String**> | The name of the destination component | [optional]
**aggregate_snapshot** | Option<[**crate::models::ConnectionStatusSnapshotDto**](ConnectionStatusSnapshotDTO.md)> |  | [optional]
**node_snapshots** | Option<[**Vec<crate::models::NodeConnectionStatusSnapshotDto>**](NodeConnectionStatusSnapshotDTO.md)> | A list of status snapshots for each node | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


