# VersionedFlowSnapshotMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link** | Option<[**crate::models::JaxbLink**](JaxbLink.md)> |  | [optional]
**bucket_identifier** | **String** | The identifier of the bucket this snapshot belongs to. | 
**flow_identifier** | **String** | The identifier of the flow this snapshot belongs to. | 
**version** | **i32** | The version of this snapshot of the flow. | 
**timestamp** | Option<**i64**> | The timestamp when the flow was saved, as milliseconds since epoch. | [optional][readonly]
**author** | Option<**String**> | The user that created this snapshot of the flow. | [optional][readonly]
**comments** | Option<**String**> | The comments provided by the user when creating the snapshot. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


