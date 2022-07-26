/*
 * NiFi Rest API
 *
 * The Rest API provides programmatic access to command and control a NiFi instance in real time. Start and                                             stop processors, monitor queues, query provenance data, and more. Each endpoint below includes a description,                                             definitions of the expected input and output, potential response codes, and the authorizations required                                             to invoke each service.
 *
 * The version of the OpenAPI document: 1.16.0
 * Contact: dev@nifi.apache.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProvenanceEventDto {
    /// The event uuid.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The event id. This is a one up number thats unique per node.
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// The timestamp of the event.
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// The event duration in milliseconds.
    #[serde(rename = "eventDuration", skip_serializing_if = "Option::is_none")]
    pub event_duration: Option<i64>,
    /// The duration since the lineage began, in milliseconds.
    #[serde(rename = "lineageDuration", skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// The type of the event.
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The uuid of the flowfile for the event.
    #[serde(rename = "flowFileUuid", skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The size of the flowfile for the event.
    #[serde(rename = "fileSize", skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    /// The size of the flowfile in bytes for the event.
    #[serde(rename = "fileSizeBytes", skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<i64>,
    /// The identifier for the node where the event originated.
    #[serde(rename = "clusterNodeId", skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The label for the node where the event originated.
    #[serde(rename = "clusterNodeAddress", skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The id of the group that the component resides in. If the component is no longer in the flow, the group id will not be set.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the component that generated the event.
    #[serde(rename = "componentId", skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The type of the component that generated the event.
    #[serde(rename = "componentType", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The name of the component that generated the event.
    #[serde(rename = "componentName", skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The source system flowfile id.
    #[serde(rename = "sourceSystemFlowFileId", skip_serializing_if = "Option::is_none")]
    pub source_system_flow_file_id: Option<String>,
    /// The alternate identifier uri for the fileflow for the event.
    #[serde(rename = "alternateIdentifierUri", skip_serializing_if = "Option::is_none")]
    pub alternate_identifier_uri: Option<String>,
    /// The attributes of the flowfile for the event.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<crate::models::AttributeDto>>,
    /// The parent uuids for the event.
    #[serde(rename = "parentUuids", skip_serializing_if = "Option::is_none")]
    pub parent_uuids: Option<Vec<String>>,
    /// The child uuids for the event.
    #[serde(rename = "childUuids", skip_serializing_if = "Option::is_none")]
    pub child_uuids: Option<Vec<String>>,
    /// The source/destination system uri if the event was a RECEIVE/SEND.
    #[serde(rename = "transitUri", skip_serializing_if = "Option::is_none")]
    pub transit_uri: Option<String>,
    /// The relationship to which the flowfile was routed if the event is of type ROUTE.
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// The event details.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Whether the input and output content claim is the same.
    #[serde(rename = "contentEqual", skip_serializing_if = "Option::is_none")]
    pub content_equal: Option<bool>,
    /// Whether the input content is still available.
    #[serde(rename = "inputContentAvailable", skip_serializing_if = "Option::is_none")]
    pub input_content_available: Option<bool>,
    /// The section in which the input content claim lives.
    #[serde(rename = "inputContentClaimSection", skip_serializing_if = "Option::is_none")]
    pub input_content_claim_section: Option<String>,
    /// The container in which the input content claim lives.
    #[serde(rename = "inputContentClaimContainer", skip_serializing_if = "Option::is_none")]
    pub input_content_claim_container: Option<String>,
    /// The identifier of the input content claim.
    #[serde(rename = "inputContentClaimIdentifier", skip_serializing_if = "Option::is_none")]
    pub input_content_claim_identifier: Option<String>,
    /// The offset into the input content claim where the flowfiles content begins.
    #[serde(rename = "inputContentClaimOffset", skip_serializing_if = "Option::is_none")]
    pub input_content_claim_offset: Option<i64>,
    /// The file size of the input content claim formatted.
    #[serde(rename = "inputContentClaimFileSize", skip_serializing_if = "Option::is_none")]
    pub input_content_claim_file_size: Option<String>,
    /// The file size of the intput content claim in bytes.
    #[serde(rename = "inputContentClaimFileSizeBytes", skip_serializing_if = "Option::is_none")]
    pub input_content_claim_file_size_bytes: Option<i64>,
    /// Whether the output content is still available.
    #[serde(rename = "outputContentAvailable", skip_serializing_if = "Option::is_none")]
    pub output_content_available: Option<bool>,
    /// The section in which the output content claim lives.
    #[serde(rename = "outputContentClaimSection", skip_serializing_if = "Option::is_none")]
    pub output_content_claim_section: Option<String>,
    /// The container in which the output content claim lives.
    #[serde(rename = "outputContentClaimContainer", skip_serializing_if = "Option::is_none")]
    pub output_content_claim_container: Option<String>,
    /// The identifier of the output content claim.
    #[serde(rename = "outputContentClaimIdentifier", skip_serializing_if = "Option::is_none")]
    pub output_content_claim_identifier: Option<String>,
    /// The offset into the output content claim where the flowfiles content begins.
    #[serde(rename = "outputContentClaimOffset", skip_serializing_if = "Option::is_none")]
    pub output_content_claim_offset: Option<i64>,
    /// The file size of the output content claim formatted.
    #[serde(rename = "outputContentClaimFileSize", skip_serializing_if = "Option::is_none")]
    pub output_content_claim_file_size: Option<String>,
    /// The file size of the output content claim in bytes.
    #[serde(rename = "outputContentClaimFileSizeBytes", skip_serializing_if = "Option::is_none")]
    pub output_content_claim_file_size_bytes: Option<i64>,
    /// Whether or not replay is available.
    #[serde(rename = "replayAvailable", skip_serializing_if = "Option::is_none")]
    pub replay_available: Option<bool>,
    /// Explanation as to why replay is unavailable.
    #[serde(rename = "replayExplanation", skip_serializing_if = "Option::is_none")]
    pub replay_explanation: Option<String>,
    /// The identifier of the queue/connection from which the flowfile was pulled to genereate this event. May be null if the queue/connection is unknown or the flowfile was generated from this event.
    #[serde(rename = "sourceConnectionIdentifier", skip_serializing_if = "Option::is_none")]
    pub source_connection_identifier: Option<String>,
}

impl ProvenanceEventDto {
    pub fn new() -> ProvenanceEventDto {
        ProvenanceEventDto {
            id: None,
            event_id: None,
            event_time: None,
            event_duration: None,
            lineage_duration: None,
            event_type: None,
            flow_file_uuid: None,
            file_size: None,
            file_size_bytes: None,
            cluster_node_id: None,
            cluster_node_address: None,
            group_id: None,
            component_id: None,
            component_type: None,
            component_name: None,
            source_system_flow_file_id: None,
            alternate_identifier_uri: None,
            attributes: None,
            parent_uuids: None,
            child_uuids: None,
            transit_uri: None,
            relationship: None,
            details: None,
            content_equal: None,
            input_content_available: None,
            input_content_claim_section: None,
            input_content_claim_container: None,
            input_content_claim_identifier: None,
            input_content_claim_offset: None,
            input_content_claim_file_size: None,
            input_content_claim_file_size_bytes: None,
            output_content_available: None,
            output_content_claim_section: None,
            output_content_claim_container: None,
            output_content_claim_identifier: None,
            output_content_claim_offset: None,
            output_content_claim_file_size: None,
            output_content_claim_file_size_bytes: None,
            replay_available: None,
            replay_explanation: None,
            source_connection_identifier: None,
        }
    }
}


