# ProvenanceNodeDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the node. | [optional]
**flow_file_uuid** | Option<**String**> | The uuid of the flowfile associated with the provenance event. | [optional]
**parent_uuids** | Option<**Vec<String>**> | The uuid of the parent flowfiles of the provenance event. | [optional]
**child_uuids** | Option<**Vec<String>**> | The uuid of the childrent flowfiles of the provenance event. | [optional]
**cluster_node_identifier** | Option<**String**> | The identifier of the node that this event/flowfile originated from. | [optional]
**_type** | Option<**String**> | The type of the node. | [optional]
**event_type** | Option<**String**> | If the type is EVENT, this is the type of event. | [optional]
**millis** | Option<**i64**> | The timestamp of the node in milliseconds. | [optional]
**timestamp** | Option<**String**> | The timestamp of the node formatted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


