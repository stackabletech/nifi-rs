# NodeStatusSnapshotsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**String**> | The id of the node. | [optional]
**address** | Option<**String**> | The node's host/ip address. | [optional]
**api_port** | Option<**i32**> | The port the node is listening for API requests. | [optional]
**status_snapshots** | Option<[**Vec<crate::models::StatusSnapshotDto>**](StatusSnapshotDTO.md)> | A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


