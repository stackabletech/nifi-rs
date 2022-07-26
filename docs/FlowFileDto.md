# FlowFileDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | Option<**String**> | The URI that can be used to access this FlowFile. | [optional]
**uuid** | Option<**String**> | The FlowFile UUID. | [optional]
**filename** | Option<**String**> | The FlowFile filename. | [optional]
**position** | Option<**i32**> | The FlowFile's position in the queue. | [optional]
**size** | Option<**i64**> | The FlowFile file size. | [optional]
**queued_duration** | Option<**i64**> | How long this FlowFile has been enqueued. | [optional]
**lineage_duration** | Option<**i64**> | Duration since the FlowFile's greatest ancestor entered the flow. | [optional]
**penalty_expires_in** | Option<**i64**> | How long in milliseconds until the FlowFile penalty expires. | [optional]
**cluster_node_id** | Option<**String**> | The id of the node where this FlowFile resides. | [optional]
**cluster_node_address** | Option<**String**> | The label for the node where this FlowFile resides. | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | The FlowFile attributes. | [optional]
**content_claim_section** | Option<**String**> | The section in which the content claim lives. | [optional]
**content_claim_container** | Option<**String**> | The container in which the content claim lives. | [optional]
**content_claim_identifier** | Option<**String**> | The identifier of the content claim. | [optional]
**content_claim_offset** | Option<**i64**> | The offset into the content claim where the flowfile's content begins. | [optional]
**content_claim_file_size** | Option<**String**> | The file size of the content claim formatted. | [optional]
**content_claim_file_size_bytes** | Option<**i64**> | The file size of the content claim in bytes. | [optional]
**penalized** | Option<**bool**> | If the FlowFile is penalized. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


