# ProvenanceEventDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The event uuid. | [optional]
**event_id** | Option<**i64**> | The event id. This is a one up number thats unique per node. | [optional]
**event_time** | Option<**String**> | The timestamp of the event. | [optional]
**event_duration** | Option<**i64**> | The event duration in milliseconds. | [optional]
**lineage_duration** | Option<**i64**> | The duration since the lineage began, in milliseconds. | [optional]
**event_type** | Option<**String**> | The type of the event. | [optional]
**flow_file_uuid** | Option<**String**> | The uuid of the flowfile for the event. | [optional]
**file_size** | Option<**String**> | The size of the flowfile for the event. | [optional]
**file_size_bytes** | Option<**i64**> | The size of the flowfile in bytes for the event. | [optional]
**cluster_node_id** | Option<**String**> | The identifier for the node where the event originated. | [optional]
**cluster_node_address** | Option<**String**> | The label for the node where the event originated. | [optional]
**group_id** | Option<**String**> | The id of the group that the component resides in. If the component is no longer in the flow, the group id will not be set. | [optional]
**component_id** | Option<**String**> | The id of the component that generated the event. | [optional]
**component_type** | Option<**String**> | The type of the component that generated the event. | [optional]
**component_name** | Option<**String**> | The name of the component that generated the event. | [optional]
**source_system_flow_file_id** | Option<**String**> | The source system flowfile id. | [optional]
**alternate_identifier_uri** | Option<**String**> | The alternate identifier uri for the fileflow for the event. | [optional]
**attributes** | Option<[**Vec<crate::models::AttributeDto>**](AttributeDTO.md)> | The attributes of the flowfile for the event. | [optional]
**parent_uuids** | Option<**Vec<String>**> | The parent uuids for the event. | [optional]
**child_uuids** | Option<**Vec<String>**> | The child uuids for the event. | [optional]
**transit_uri** | Option<**String**> | The source/destination system uri if the event was a RECEIVE/SEND. | [optional]
**relationship** | Option<**String**> | The relationship to which the flowfile was routed if the event is of type ROUTE. | [optional]
**details** | Option<**String**> | The event details. | [optional]
**content_equal** | Option<**bool**> | Whether the input and output content claim is the same. | [optional]
**input_content_available** | Option<**bool**> | Whether the input content is still available. | [optional]
**input_content_claim_section** | Option<**String**> | The section in which the input content claim lives. | [optional]
**input_content_claim_container** | Option<**String**> | The container in which the input content claim lives. | [optional]
**input_content_claim_identifier** | Option<**String**> | The identifier of the input content claim. | [optional]
**input_content_claim_offset** | Option<**i64**> | The offset into the input content claim where the flowfiles content begins. | [optional]
**input_content_claim_file_size** | Option<**String**> | The file size of the input content claim formatted. | [optional]
**input_content_claim_file_size_bytes** | Option<**i64**> | The file size of the intput content claim in bytes. | [optional]
**output_content_available** | Option<**bool**> | Whether the output content is still available. | [optional]
**output_content_claim_section** | Option<**String**> | The section in which the output content claim lives. | [optional]
**output_content_claim_container** | Option<**String**> | The container in which the output content claim lives. | [optional]
**output_content_claim_identifier** | Option<**String**> | The identifier of the output content claim. | [optional]
**output_content_claim_offset** | Option<**i64**> | The offset into the output content claim where the flowfiles content begins. | [optional]
**output_content_claim_file_size** | Option<**String**> | The file size of the output content claim formatted. | [optional]
**output_content_claim_file_size_bytes** | Option<**i64**> | The file size of the output content claim in bytes. | [optional]
**replay_available** | Option<**bool**> | Whether or not replay is available. | [optional]
**replay_explanation** | Option<**String**> | Explanation as to why replay is unavailable. | [optional]
**source_connection_identifier** | Option<**String**> | The identifier of the queue/connection from which the flowfile was pulled to genereate this event. May be null if the queue/connection is unknown or the flowfile was generated from this event. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


