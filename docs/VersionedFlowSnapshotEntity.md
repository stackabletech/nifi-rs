# VersionedFlowSnapshotEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**versioned_flow_snapshot** | Option<[**crate::models::VersionedFlowSnapshot**](VersionedFlowSnapshot.md)> |  | [optional]
**process_group_revision** | Option<[**crate::models::RevisionDto**](RevisionDTO.md)> |  | [optional]
**registry_id** | Option<**String**> | The ID of the Registry that this flow belongs to | [optional]
**update_descendant_versioned_flows** | Option<**bool**> | If the Process Group to be updated has a child or descendant Process Group that is also under Version Control, this specifies whether or not the contents of that child/descendant Process Group should be updated. | [optional]
**disconnected_node_acknowledged** | Option<**bool**> | Acknowledges that this node is disconnected to allow for mutable requests to proceed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


